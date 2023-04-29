#[allow(dead_code)]
#[derive(Clone)]
pub enum PeerModel {
    Dictionary(Vec<Peer>),
    String(String),
}

#[allow(dead_code)]
#[derive(Clone)]
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

impl Response {
    pub fn builder(tracker_id: &str, peers: PeerModel) -> ResponseBuilder {
        ResponseBuilder {
            failure_reason: None,
            warning_message: None,
            interval: 3600,
            min_interval: None,
            tracker_id: tracker_id.to_string(),
            complete: 0,
            incomplete: 0,
            peers,
        }
    }
}

/// Tracker response builder.
#[allow(dead_code)]
pub struct ResponseBuilder {
    failure_reason: Option<String>,
    warning_message: Option<String>,
    interval: u64,
    min_interval: Option<String>,
    tracker_id: String,
    complete: u64,
    incomplete: u64,
    peers: PeerModel,
}

impl ResponseBuilder {
    pub fn build(&mut self) -> Response {
        Response {
            failure_reason: self.failure_reason.clone(),
            warning_message: self.warning_message.clone(),
            interval: self.interval,
            min_interval: self.min_interval.clone(),
            tracker_id: self.tracker_id.clone(),
            complete: self.complete,
            incomplete: self.incomplete,
            peers: self.peers.clone(),
        }
    }
}
