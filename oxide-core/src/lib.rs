mod error;
mod tracker;
mod utils;
mod metainfo;

pub use metainfo::MetaInfo;
pub use tracker::{Query, Event};

#[cfg(test)]
mod tests {
    use super::tracker;
    use super::MetaInfo;
    use hex_literal::hex;
    use rand::distributions::{Alphanumeric, DistString};
    use std::borrow::Cow;

    const DEBIAN_URL: &'static str = "https://cdimage.debian.org/cdimage/archive/11.7.0/amd64/bt-cd/debian-11.7.0-amd64-netinst.iso.torrent";

    #[test]
    fn parse_torrent_file() {
        let metainfo = MetaInfo::from_url(DEBIAN_URL).unwrap();
        let info = metainfo.info();

        assert_eq!(
            metainfo.announce(),
            "http://bttracker.debian.org:6969/announce"
        );
        assert_eq!(info.name(), "debian-11.7.0-amd64-netinst.iso");
        assert_eq!(info.length(), Some(407896064));
        assert_eq!(info.piece_length(), 262144);
        assert_eq!(
            info.hash()[..],
            hex!("6f84758b0ddd8dc05840bf932a77935d8b5b8b93")
        );
    }
    #[test]
    fn send_tracker_request() {
        let metainfo = MetaInfo::from_url(DEBIAN_URL).unwrap();
        let info = metainfo.info();

        let peer_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 20);
        let query = tracker::Query::new(info.hash(), &peer_id, info.length().unwrap());

        let client = reqwest::Client::new();
        let res = client
            .get(metainfo.announce())
            .query(&query)
            .build()
            .unwrap();
        let mut pairs = res.url().query_pairs();

        assert_eq!(
            pairs.next(),
            Some((
                Cow::Borrowed("info_hash"),
                Cow::Borrowed("o%84u%8b%d%dd%8d%c0X%40%bf%93%2aw%93%5d%8b%5b%8b%93")
            ))
        );
        assert_eq!(
            pairs.next(),
            Some((Cow::Borrowed("peer_id"), Cow::from(peer_id)))
        );
        assert_eq!(
            pairs.next(),
            Some((Cow::Borrowed("port"), Cow::Borrowed("6881")))
        );
        assert_eq!(
            pairs.next(),
            Some((Cow::Borrowed("uploaded"), Cow::Borrowed("0")))
        );
        assert_eq!(
            pairs.next(),
            Some((Cow::Borrowed("downloaded"), Cow::Borrowed("0")))
        );
        assert_eq!(
            pairs.next(),
            Some((
                Cow::Borrowed("left"),
                Cow::from(info.length().unwrap().to_string())
            ))
        );
    }
}
