pub enum Event {
    Started,
    Completed,
    Stopped,
}

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

#[allow(dead_code)]
pub struct TrackerRequest {
    info_hash: String,
    peer_id: String,
    port: u32,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    compact: bool,
    no_peer_id: bool,
    event: Option<Event>,
    ip: Option<String>,
    numwant: Option<u64>,
    key: Option<String>,
    trackerid: Option<String>,
}

#[allow(dead_code)]
pub struct TrackerResponse {
    failure_reason: Option<String>,
    warning_message: Option<String>,
    interval: u64,
    min_interval: Option<String>,
    tracker_id: String,
    complete: u64,
    incomplete: u64,
    peers: PeerModel,
}
