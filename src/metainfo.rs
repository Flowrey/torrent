use std::fs;

use serde::Deserialize;
use serde_bencode::de;

use crate::error::Error;

#[derive(Clone, Deserialize, Debug)]
#[allow(dead_code)]
pub struct File {
    length: u64,
    md5sum: Option<String>,
    path: Vec<String>,
}

impl File {
    pub fn new(path: Vec<String>, length: u64, md5sum: Option<String>) -> File {
        File {
            path,
            length,
            md5sum,
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Info {
    #[serde(rename = "piece length")]
    piece_length: u64,
    #[serde(with = "serde_bytes")]
    pieces: Vec<u8>,
    private: Option<bool>,

    name: String,

    // Info in Single File Mode
    length: Option<u64>, // Exclusive with files

    // Info in Multiple File Mode
    files: Option<Vec<File>>, // Exclusive with length
}

impl Info {
    pub fn new_single_file(pieces: &str, name: &str, length: u64) -> Info {
        Self::single_file_builder(pieces, name, length).build()
    }

    pub fn new_multiple_file(pieces: &str, name: &str, files: Vec<File>) -> Info {
        Self::multiple_file_builder(pieces, name, files).build()
    }

    pub fn single_file_builder(pieces: &str, name: &str, length: u64) -> InfoBuilder {
        InfoBuilder {
            piece_length: 512,
            pieces: pieces.as_bytes().to_vec(),
            private: None,
            name: name.to_string(),
            length: Some(length),
            files: None,
        }
    }

    pub fn multiple_file_builder(pieces: &str, name: &str, files: Vec<File>) -> InfoBuilder {
        InfoBuilder {
            piece_length: 512,
            pieces: pieces.as_bytes().to_vec(),
            private: None,
            name: name.to_string(),
            length: None,
            files: Some(files),
        }
    }
}

pub struct InfoBuilder {
    piece_length: u64,
    pieces: Vec<u8>,
    private: Option<bool>,

    name: String,

    // Info in Single File Mode
    length: Option<u64>, // Exclusive with files

    // Info in Multiple File Mode
    files: Option<Vec<File>>, // Exclusive with length
}

impl InfoBuilder {
    pub fn build(&mut self) -> Info {
        Info {
            piece_length: self.piece_length,
            pieces: self.pieces.clone(),
            private: self.private,
            name: self.name.clone(),
            length: self.length,
            files: self.files.clone(),
        }
    }

    pub fn piece_length(&mut self, piece_length: u64) -> &mut Self {
        self.piece_length = piece_length;
        self
    }

    pub fn private(&mut self, private: bool) -> &mut Self {
        self.private = Some(private);
        self
    }
}

#[derive(Clone, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Metainfo {
    info: Info,
    announce: String,
    #[serde(rename = "announce-list")]
    announce_list: Option<String>,
    #[serde(rename = "creation date")]
    creation_date: Option<u64>,
    comment: Option<String>,
    #[serde(rename = "created by")]
    created_by: Option<String>,
    encoding: Option<String>,
}

impl Metainfo {
    pub fn new(info: Info, announce: &str) -> Metainfo {
        Self::builder(info, announce).build()
    }

    pub fn try_from_bytes(bytes: &[u8]) -> Result<Metainfo, Error> {
        de::from_bytes::<Metainfo>(bytes).map_err(Error::DeError)
    }

    pub fn try_from_torrent(path: &str) -> Result<Metainfo, Error> {
        let bytes = fs::read(path)?;
        Self::try_from_bytes(&bytes)
    }

    pub fn builder(info: Info, announce: &str) -> MetainfoBuilder {
        MetainfoBuilder {
            info,
            announce: announce.to_string(),
            announce_list: None,
            creation_date: None,
            comment: None,
            created_by: None,
            encoding: None,
        }
    }

    /// The announce URL of the tracker
    pub fn annouce(self) -> String {
        self.announce
    }
}

pub struct MetainfoBuilder {
    info: Info,
    announce: String,
    announce_list: Option<String>,
    creation_date: Option<u64>,
    comment: Option<String>,
    created_by: Option<String>,
    encoding: Option<String>,
}

impl MetainfoBuilder {
    pub fn announce_list(&mut self, announce_list: &str) -> &mut Self {
        self.announce_list = Some(announce_list.to_string());
        self
    }

    pub fn creation_date(&mut self, creation_date: u64) -> &mut Self {
        self.creation_date = Some(creation_date);
        self
    }

    pub fn comment(&mut self, comment: &str) -> &mut Self {
        self.comment = Some(comment.to_string());
        self
    }

    pub fn created_by(&mut self, created_by: &str) -> &mut Self {
        self.created_by = Some(created_by.to_string());
        self
    }

    pub fn encoding(&mut self, encoding: &str) -> &mut Self {
        self.encoding = Some(encoding.to_string());
        self
    }

    pub fn build(&mut self) -> Metainfo {
        Metainfo {
            info: self.info.clone(),
            announce: self.announce.clone(),
            announce_list: self.announce_list.clone(),
            creation_date: self.creation_date,
            comment: self.comment.clone(),
            created_by: self.created_by.clone(),
            encoding: self.encoding.clone(),
        }
    }
}
