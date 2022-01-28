use serde::{Deserialize, Serialize};

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
}
