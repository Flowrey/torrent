use serde::{
    ser::{self, SerializeStruct},
    Serialize,
};

use super::urlencode;

#[allow(dead_code)]
#[derive(Serialize, Clone)]
pub enum Event {
    Started,
    Completed,
    Stopped,
}

#[allow(dead_code)]
/// Parameters used to request the tracker.
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

impl Serialize for Request {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Request", 13)?;

        let hash = hex::decode(&self.info_hash)
            .map_err(|_| ser::Error::custom("failed to decode info_hash to hex"))?;
        state.serialize_field("info_hash", &urlencode(&hash))?;
        state.serialize_field("peer_id", &self.peer_id)?;
        state.serialize_field("port", &self.port)?;
        state.serialize_field("uploaded", &self.uploaded)?;
        state.serialize_field("downloaded", &self.downloaded)?;
        state.serialize_field("left", &self.left)?;
        state.serialize_field("compact", &self.compact)?;
        state.serialize_field("no_peer_id", &self.no_peer_id)?;
        state.serialize_field("event", &self.event)?;
        state.serialize_field("ip", &self.ip)?;
        state.serialize_field("numwant", &self.numwant)?;
        state.serialize_field("key", &self.key)?;
        state.serialize_field("trackerid", &self.trackerid)?;
        state.end()
    }
}

/// Builder to create a custom `Request`.
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
    /// Constructs a new `Request`.
    pub fn new(info_hash: &str, event: Option<Event>) -> Request {
        Request::builder(info_hash.to_string(), event).build()
    }

    /// Creates a `RequestBuilder` to configure a custom `Request`.
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
    /// Build a `Request` that use the specified configuration.
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

    /// Unique ID for the client.
    pub fn peer_id(&mut self, peer_id: String) -> &mut RequestBuilder {
        self.peer_id = peer_id;
        self
    }

    /// The port number thaht the client is listening on.
    pub fn port(&mut self, port: u32) -> &mut RequestBuilder {
        self.port = port;
        self
    }

    /// The total amount uploaded.
    pub fn uploaded(&mut self, uploaded: u64) -> &mut RequestBuilder {
        self.uploaded = uploaded;
        self
    }

    /// The total amount downloaded.
    pub fn downloaded(&mut self, downloaded: u64) -> &mut RequestBuilder {
        self.downloaded = downloaded;
        self
    }

    /// The number of bytes this client still has to download.
    pub fn left(&mut self, left: u64) -> &mut RequestBuilder {
        self.left = left;
        self
    }

    /// Indicates that the tracker can omit peer id.
    pub fn no_peer_id(&mut self, no_peer_id: bool) -> &mut RequestBuilder {
        self.no_peer_id = no_peer_id;
        self
    }

    /// The trupe IP address of the client machine.
    pub fn ip(&mut self, ip: String) -> &mut RequestBuilder {
        self.ip = Some(ip);
        self
    }

    /// Number of peers that the client would like to receive from the
    /// tracker.
    pub fn numwant(&mut self, numwant: u64) -> &mut RequestBuilder {
        self.numwant = Some(numwant);
        self
    }

    /// An additional identification that is not shared with any
    /// other peers.
    pub fn key(&mut self, key: String) -> &mut RequestBuilder {
        self.key = Some(key);
        self
    }

    /// If a previous annouce contained a tracker id, it shoud be set here.
    pub fn trackerid(&mut self, trackerid: String) -> &mut RequestBuilder {
        self.key = Some(trackerid);
        self
    }
}
