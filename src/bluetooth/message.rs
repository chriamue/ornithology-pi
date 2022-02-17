use serde::{Deserialize, Serialize};

use crate::Sighting;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "op")]
pub enum Message {
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "pong")]
    Pong,
    #[serde(rename = "count_request")]
    CountRequest,
    #[serde(rename = "count_response")]
    CountResponse { count: u64 },
    #[serde(rename = "sighting_ids_request")]
    SightingIdsRequest,
    #[serde(rename = "sighting_ids_response")]
    SightingIdsResponse { ids: Vec<String> },
    #[serde(rename = "last_request")]
    LastRequest,
    #[serde(rename = "last_response")]
    LastResponse { last: Sighting },
    #[serde(rename = "sighting_request")]
    SightingRequest { uuid: String },
    #[serde(rename = "sighting_response")]
    SightingResponse { sighting: Sighting },
    #[serde(rename = "image_request")]
    ImageRequest { uuid: String },
    #[serde(rename = "image_response")]
    ImageResponse { uuid: String, base64: String },
}
