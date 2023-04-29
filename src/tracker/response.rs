#[allow(dead_code)]
pub enum PeerModel {
    Dictionary(Vec<Peer>),
    String(String),
}

#[allow(dead_code)]
pub struct Peer {
    peer_id: String,
    ip: String,
    port: u32,
}

/// Tracker response.
#[allow(dead_code)]
pub struct Response {
    failure_reason: Option<String>,
    warning_message: Option<String>,
    interval: u64,
    min_interval: Option<String>,
    tracker_id: String,
    complete: u64,
    incomplete: u64,
    peers: PeerModel,
}
