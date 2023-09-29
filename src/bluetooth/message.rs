use serde::{Deserialize, Serialize};

use crate::Sighting;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Message {
    Ping,
    Pong,
    CountRequest,
    CountResponse { count: u64 },
    SightingIdsRequest,
    SightingIdsResponse { ids: Vec<String> },
    LastRequest,
    LastResponse { last: Sighting },
    SightingRequest { uuid: String },
    RemoveSightingRequest { uuid: String },
    SightingResponse { sighting: Sighting },
    ImageRequest { uuid: String },
    ImageResponse { uuid: String, base64: String },
}
