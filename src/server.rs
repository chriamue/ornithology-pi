#[cfg(feature = "detect")]
use crate::BirdDetector;
use crate::{sighting::save_to_file, Config};
use crate::{Capture, MJpeg, Sighting, WebCam};
use axum::{
    body::StreamBody,
    extract,
    extract::Json,
    response::{IntoResponse, Response},
    routing::{delete, get},
    Extension, Router,
};
use hyper::Method;
use hyper::{header::CONTENT_DISPOSITION, Body, StatusCode};
use hyper::{header::CONTENT_TYPE, Server};
use serde_json::{json, Value};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, net::SocketAddr};
use tokio_util::io::ReaderStream;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

pub type SightingsContainer = Arc<Mutex<Vec<Sighting>>>;

#[derive(Debug)]
enum AppError {
    LockError,
    NotFound,
    ParseError,
    IoError,
    ResponseBuildError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::LockError => (StatusCode::INTERNAL_SERVER_ERROR, "Lock error"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::ParseError => (StatusCode::BAD_REQUEST, "Parse error"),
            AppError::IoError => (StatusCode::INTERNAL_SERVER_ERROR, "IO error"),
            AppError::ResponseBuildError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Response build error")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[derive(Clone)]
pub struct DetectorState {
    #[cfg(feature = "detect")]
    pub mutex: Arc<Mutex<BirdDetector>>,
}

async fn generate_204() -> impl IntoResponse {
    (StatusCode::NO_CONTENT, "")
}

async fn get_sightings(
    Extension(sightings): Extension<SightingsContainer>,
    extract::Query(params): extract::Query<HashMap<String, String>>,
) -> Result<Json<Value>, AppError> {
    let start: Option<usize> = params
        .get("start")
        .map(|x| x.parse::<usize>())
        .transpose()
        .map_err(|_| AppError::ParseError)?;
    let end: Option<usize> = params
        .get("end")
        .map(|x| x.parse::<usize>())
        .transpose()
        .map_err(|_| AppError::ParseError)?;

    let sightings = sightings.lock().map_err(|_| AppError::LockError)?;
    let sightings = match (start, end) {
        (Some(start), Some(end)) => {
            let end = end.min(sightings.len()).max(0);
            sightings[start.max(0).min(end)..end].to_vec()
        }
        (Some(start), None) => sightings[start..].to_vec(),
        _ => sightings.to_vec(),
    };
    Ok(Json(json!(sightings)))
}

async fn webcam(
    Extension(capture): Extension<Arc<Mutex<WebCam>>>,
) -> Result<Response<Body>, AppError> {
    let capture: Arc<Mutex<WebCam>> = capture.clone();

    let mjpeg = MJpeg::new(capture);
    let body = Body::wrap_stream(mjpeg);

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "multipart/x-mixed-replace; boundary=frame")
        .body(body)
        .map_err(|_| AppError::ResponseBuildError)
}

async fn frame(
    Extension(capture): Extension<Arc<Mutex<WebCam>>>,
) -> Result<Response<Body>, AppError> {
    let frame = {
        let mut capture = capture.lock().map_err(|_| AppError::LockError)?;
        capture.bytes_jpeg().unwrap()
    };
    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime::IMAGE_JPEG.as_ref())
        .body(Body::from(frame))
        .map_err(|_| AppError::ResponseBuildError)
}

async fn sighting(
    Extension(sightings): Extension<SightingsContainer>,
    extract::Path(id): extract::Path<String>,
) -> Result<Response<StreamBody<ReaderStream<tokio::fs::File>>>, AppError> {
    let filename = {
        let sightings = sightings.lock().map_err(|_| AppError::LockError)?;
        let sighting = sightings
            .iter()
            .filter(|sighting| sighting.uuid == id)
            .last()
            .cloned()
            .ok_or(AppError::NotFound)?;
        format!("{}_{}.jpg", sighting.species, sighting.uuid)
    };

    let file_path = Path::new("sightings/").join(&filename);
    let file = tokio::fs::File::open(file_path)
        .await
        .map_err(|_| AppError::IoError)?;
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    Response::builder()
        .status(StatusCode::OK)
        .header(
            CONTENT_DISPOSITION,
            format!("attachment; filename={}", filename),
        )
        .body(body)
        .map_err(|_| AppError::ResponseBuildError)
}

async fn delete_sighting(
    Extension(sightings): Extension<SightingsContainer>,
    extract::Path(id): extract::Path<String>,
) -> Result<(), AppError> {
    let filename = {
        let sightings = sightings.lock().map_err(|_| AppError::LockError)?;
        let sighting = sightings
            .iter()
            .filter(|sighting| sighting.uuid == id)
            .last()
            .cloned()
            .ok_or(AppError::NotFound)?;
        format!("{}_{}.jpg", sighting.species, sighting.uuid)
    };

    tokio::fs::remove_file(Path::new("sightings/").join(filename))
        .await
        .map_err(|_| AppError::IoError)?;
    let mut sightings = sightings.lock().map_err(|_| AppError::LockError)?;
    let index = sightings
        .iter()
        .position(|x| x.uuid == id)
        .ok_or(AppError::NotFound)?;
    sightings.remove(index);
    save_to_file(sightings.to_vec(), "sightings/sightings.db").map_err(|_| AppError::IoError)?;
    Ok(())
}

pub async fn server(config: &Config, sightings: SightingsContainer, capture: Arc<Mutex<WebCam>>) {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(tower_http::cors::Any);

    let serve_dir =
        ServeDir::new("app/dist").not_found_service(ServeFile::new("app/dist/index.html"));

    let app = Router::new()
        .nest_service("/", serve_dir.clone())
        .route("/generate_204", get(generate_204))
        .route("/sightings", get(get_sightings))
        .route("/webcam", get(webcam))
        .route("/frame", get(frame))
        .route("/sightings/:id", get(sighting))
        .route("/sightings/:id", delete(delete_sighting))
        .layer(Extension(sightings.clone()))
        .layer(Extension(capture))
        .layer(cors);

    let addr = SocketAddr::new(config.server.address.parse().unwrap(), config.server.port);
    let server = Server::bind(&addr).serve(app.into_make_service());

    println!("listening on {}", addr);
    server.await.unwrap();
}
