mod error;
mod tracker;
mod utils;
mod metainfo;

pub use metainfo::MetaInfo;
pub use tracker::{Query, Event};
pub use utils::urlencode;

#[cfg(test)]
mod tests {
    use crate::utils::urlencode;

    use super::tracker;
    use super::MetaInfo;
    use hex_literal::hex;
    use rand::distributions::{Alphanumeric, DistString};
    use reqwest::StatusCode;
    use url::Url;
    use std::borrow::Cow;

    const DEBIAN_URL: &'static str = "https://cdimage.debian.org/cdimage/archive/11.7.0/amd64/bt-cd/debian-11.7.0-amd64-netinst.iso.torrent";

    #[tokio::test]
    async fn parse_torrent_file() {
        let metainfo = MetaInfo::from_url(DEBIAN_URL).await.unwrap();
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

    #[tokio::test]
    async fn test_tracker_query_options() {
        let metainfo = MetaInfo::from_url(DEBIAN_URL).await.unwrap();
        let info = metainfo.info();

        let peer_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 20);
        let query = tracker::Query::new(&peer_id, info.length().unwrap());

        let mut url = Url::parse(&metainfo.announce()).expect("announce should be a valid url");
        url.set_query(
            Some(
                &format!(
                    "info_hash={}", 
                    &urlencode(&info.hash())
                )
            )
        );

        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .query(&query)
            .build()
            .unwrap();
        let mut pairs = res.url().query_pairs();

        pairs.next();
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
        assert_eq!(
            pairs.next(),
            Some((Cow::Borrowed("event"), Cow::Borrowed("empty")))
        );
    }

    #[tokio::test]
    async fn test_sending_requests() {
        let metainfo = MetaInfo::from_url(DEBIAN_URL).await.unwrap();
        let info = metainfo.info();

        let peer_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 20);
        let query = tracker::Query::new(&peer_id, info.length().unwrap());

        let mut url = Url::parse(&metainfo.announce()).expect("announce should be a valid url");
        url.set_query(
            Some(
                &format!(
                    "info_hash={}", 
                    &urlencode(&info.hash())
                )
            )
        );

        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .query(&query)
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK)
    }
}
