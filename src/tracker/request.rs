#[allow(dead_code)]
#[derive(Clone)]
pub enum Event {
    Started,
    Completed,
    Stopped,
}

#[allow(dead_code)]
pub struct Request {
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

pub struct RequestBuilder {
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

impl Request {
    pub fn new(info_hash: String, event: Option<Event>) -> Request {
        Request::builder(info_hash, event).build()
    }

    pub fn builder(info_hash: String, event: Option<Event>) -> RequestBuilder {
        RequestBuilder {
            info_hash,
            peer_id: "".to_string(),
            port: 6881,
            uploaded: 0,
            downloaded: 0,
            left: 0,
            compact: true,
            no_peer_id: true,
            event,
            ip: None,
            numwant: Some(50),
            key: None,
            trackerid: None,
        }
    }
}

impl RequestBuilder {
    pub fn build(&mut self) -> Request {
        Request {
            info_hash: self.info_hash.clone(),
            peer_id: self.peer_id.clone(),
            port: self.port,
            uploaded: self.uploaded,
            downloaded: self.downloaded,
            left: self.left,
            compact: self.compact,
            no_peer_id: self.no_peer_id,
            event: self.event.clone(),
            ip: self.ip.clone(),
            numwant: self.numwant,
            key: self.key.clone(),
            trackerid: self.trackerid.clone(),
        }
    }

    pub fn peer_id(&mut self, peer_id: String) -> &mut RequestBuilder {
        self.peer_id = peer_id;
        self
    }

    pub fn port(&mut self, port: u32) -> &mut RequestBuilder {
        self.port = port;
        self
    }

    pub fn uploaded(&mut self, uploaded: u64) -> &mut RequestBuilder {
        self.uploaded = uploaded;
        self
    }

    pub fn downloaded(&mut self, downloaded: u64) -> &mut RequestBuilder {
        self.downloaded = downloaded;
        self
    }

    pub fn left(&mut self, left: u64) -> &mut RequestBuilder {
        self.left = left;
        self
    }

    pub fn no_peer_id(&mut self, no_peer_id: bool) -> &mut RequestBuilder {
        self.no_peer_id = no_peer_id;
        self
    }

    pub fn ip(&mut self, ip: String) -> &mut RequestBuilder {
        self.ip = Some(ip);
        self
    }

    pub fn numwant(&mut self, numwant: u64) -> &mut RequestBuilder {
        self.numwant = Some(numwant);
        self
    }

    pub fn key(&mut self, key: String) -> &mut RequestBuilder {
        self.key = Some(key);
        self
    }

    pub fn trackerid(&mut self, trackerid: String) -> &mut RequestBuilder {
        self.key = Some(trackerid);
        self
    }
}
