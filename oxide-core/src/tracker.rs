use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")] 
pub enum Event {
    Started,
    Completed,
    Stopped,
    Empty,
}

#[derive(Debug, Clone, Serialize)]
pub struct Query {
    peer_id: String,
    ip: Option<String>,
    port: u32,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    event: Event,
}

impl Query {
    pub fn new(peer_id: &str, left: u64) -> Self {
        Query {
            peer_id: peer_id.to_string(),
            ip: None,
            port: 6881,
            uploaded: 0,
            downloaded: 0,
            left,
            event: Event::Empty,
        }
    }
}
