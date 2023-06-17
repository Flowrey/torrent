use oxide_bencode;
use reqwest;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

use crate::error::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct File {
    /// The length of the file, in bytes.
    length: usize,

    /// A list of UTF-8 encoded strings corresponding to subdirectory names,
    /// the last of which is the actual file name
    path: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Info {
    /// List of file in multi-file case.
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<Vec<File>>,

    /// If length is present then the download represents a single file.
    /// In the single file case, length maps to the length of the file in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<u64>,

    /// In the single file case, the name key is the name of a file,
    /// in the muliple file case, it's the name of a directory.
    name: String,

    /// The number of bytes in each piece the file is split into.
    #[serde(rename = "piece length")]
    piece_length: u64,

    /// String whose length is a multiple of 20.
    /// It is to be subdivided into strings of length 20, each of which is the SHA1 hash of the piece at the corresponding index.
    #[serde(with = "serde_bytes")]
    pieces: Vec<u8>,
}

impl Info {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    // Note that this is a substring of the metainfo file.
    // The info-hash must be the hash of the encoded form as found in the .torrent file,
    // which is identical to bdecoding the metainfo file,
    // extracting the info dictionary and encoding it if and only if the bdecoder fully validated the input
    // (e.g. key ordering, absence of leading zeros). Conversely
    // that means clients must either reject invalid metainfo files or extract the substring directly.
    // They must not perform a decode-encode roundtrip on invalid data.
    pub fn hash(self) -> [u8; 20] {
        let mut hasher = Sha1::new();
        let data = oxide_bencode::to_bytes(&self).unwrap();
        hasher.update(data);
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetaInfo {
    /// The URL of the tracker
    announce: String,

    /// Info dictionary
    info: Info,
}

impl MetaInfo {
    pub fn from_url(url: &str) -> Result<MetaInfo, Error> {
        let body = reqwest::blocking::get(url)?.bytes()?;
        Ok(oxide_bencode::from_bytes(&body)?)
    }

    pub fn announce(self) -> String {
        self.announce
    }

    pub fn info(&self) -> Info {
        self.info.clone()
    }
}
