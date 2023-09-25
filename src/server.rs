use crate::sighting::save_to_file;
#[cfg(feature = "detect")]
use crate::BirdDetector;
use crate::{Capture, MJpeg, Sighting, WebCam};
use axum::{
    body::StreamBody,
    extract,
    extract::Json,
    response::{IntoResponse, Response},
    routing::{delete, get},
    Extension, Router,
};
use hyper::{header::CONTENT_DISPOSITION, Body, HeaderMap, StatusCode};
use hyper::{header::CONTENT_TYPE, Server};
use serde_json::{json, Value};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, net::SocketAddr};
use tokio_util::io::ReaderStream;
use tower_http::services::{ServeDir, ServeFile};

pub type SightingsContainer = Arc<Mutex<Vec<Sighting>>>;

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
) -> extract::Json<Value> {
    let start: Option<usize> = params.get("start").map(|x| x.parse().unwrap());
    let end: Option<usize> = params.get("end").map(|x| x.parse().unwrap());

    let sightings = match sightings.lock() {
        Ok(sightings) => sightings.to_vec(),
        Err(err) => {
            println!("{}", err);
            Vec::new()
        }
    };
    let sightings = match (start, end) {
        (Some(start), Some(end)) => {
            let end = end.min(sightings.len()).max(0);
            sightings[start.max(0).min(end)..end].to_vec()
        }
        (Some(start), None) => sightings[start..].to_vec(),
        _ => sightings,
    };
    json!(sightings).into()
}

async fn webcam(Extension(capture): Extension<Arc<Mutex<WebCam>>>) -> Response<Body> {
    let capture: Arc<Mutex<WebCam>> = capture.clone();

    let mjpeg = MJpeg::new(capture);

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        "multipart/x-mixed-replace; boundary=frame".parse().unwrap(),
    );

    let body = Body::wrap_stream(mjpeg);

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "multipart/x-mixed-replace; boundary=frame")
        .body(body)
        .unwrap()
}

async fn frame(Extension(capture): Extension<Arc<Mutex<WebCam>>>) -> Response<Body> {
    let frame = {
        let mut capture = capture.lock().unwrap();
        capture.bytes_jpeg().unwrap()
    };
    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime::IMAGE_JPEG.as_ref())
        .body(Body::from(frame))
        .unwrap()
}

async fn sighting(
    Extension(sightings): Extension<SightingsContainer>,
    extract::Path(id): extract::Path<String>,
) -> Response<StreamBody<ReaderStream<tokio::fs::File>>> {
    let filename = {
        let sightings = sightings.lock().unwrap();
        let sighting = sightings
            .iter()
            .filter(|sighting| sighting.uuid == id)
            .last()
            .cloned();
        let sighting = sighting.unwrap();
        format!("{}_{}.jpg", sighting.species, sighting.uuid)
    };

    let file_path = Path::new("sightings/").join(&filename);
    let file = tokio::fs::File::open(file_path).await.ok().unwrap();
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_DISPOSITION,
        format!("attachment; filename={}", filename)
            .parse()
            .unwrap(),
    );

    Response::builder()
        .status(StatusCode::OK)
        .header(
            CONTENT_DISPOSITION,
            format!("attachment; filename={}", filename),
        )
        .body(body)
        .unwrap()
}

async fn delete_sighting(
    Extension(sightings): Extension<SightingsContainer>,
    extract::Path(id): extract::Path<String>,
) {
    let filename = {
        let sightings = sightings.lock().unwrap();
        let sighting = sightings
            .iter()
            .filter(|sighting| sighting.uuid == id)
            .last()
            .cloned();
        let sighting = sighting.unwrap();
        format!("{}_{}.jpg", sighting.species, sighting.uuid)
    };

    std::fs::remove_file(Path::new("sightings/").join(filename)).unwrap();

    let sightings = {
        let mut sightings = sightings.lock().unwrap();
        let index = sightings.iter().position(|x| x.uuid == id).unwrap();
        sightings.remove(index);
        sightings.to_vec()
    };
    save_to_file(sightings, "sightings/sightings.db").unwrap()
}

pub async fn server(sightings: SightingsContainer, capture: Arc<Mutex<WebCam>>) {
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
        .layer(Extension(capture));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let server = Server::bind(&addr).serve(app.into_make_service());

    println!("listening on {}", addr);
    server.await.unwrap();
}
