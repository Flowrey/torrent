use crate::utils::urlencode;

use serde::{Serialize, Serializer};

#[derive(Debug, Clone, Serialize)]
pub enum Event {
    Started,
    Completed,
    Stopped,
}

fn urlencode_serializer<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&urlencode(bytes))
}

#[derive(Debug, Clone, Serialize)]
pub struct Query {
    #[serde(serialize_with = "urlencode_serializer")]
    info_hash: [u8; 20],
    peer_id: String,
    ip: Option<String>,
    port: u32,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    event: Option<Event>,
}

impl Query {
    pub fn new(info_hash: [u8; 20], peer_id: &str, left: u64) -> Self {
        Query {
            info_hash,
            peer_id: peer_id.to_string(),
            ip: None,
            port: 6881,
            uploaded: 0,
            downloaded: 0,
            left,
            event: None,
        }
    }
}
