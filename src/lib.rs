pub mod metainfo;
pub mod tracker;
pub mod error;

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::metainfo::{Info, Metainfo};
    use super::tracker::Request;
    use base64::Engine;
    use base64::engine::general_purpose;
    use reqwest;

    #[test]
    fn build_metainfo() {
        Metainfo::new(
            Info::new_single_file("e04a20f7b16636fc5889201e73ac8625", "debian.iso", 100),
            "http://localhost:8080",
        );
    }

    #[test]
    fn build_metainfo_from_torrent() {
        let b64_torrent = "ZDg6YW5ub3VuY2UzMDpodHRwOi8vbG9jYWxob3N0OjMwMDAvYW5ub3VuY2UxMDpjcmVhdGVkIGJ5MTM6bWt0b3JyZW50IDEuMTEzOmNyZWF0aW9uIGRhdGVpMTY4Mzc0NDk3N2U0OmluZm9kNjpsZW5ndGhpMTJlNDpuYW1lNDp0ZXN0MTI6cGllY2UgbGVuZ3RoaTI2MjE0NGU2OnBpZWNlczIwOvzPTixWdm/+2jpCJFIka+4vtk0WZWU=";
        let bytes = general_purpose::STANDARD.decode(b64_torrent).unwrap();
        let metainfo = Metainfo::try_from_bytes(&bytes).unwrap();
        assert_eq!(metainfo.annouce(), "http://localhost:3000/announce")
    }

    #[test]
    fn send_tracker_request() {
        let metainfo = Metainfo::new(
            Info::new_single_file("e04a20f7b16636fc5889201e73ac8625", "debian.iso", 100),
            "http://127.0.0.1:3000/announce",
        );

        let query = Request::new("e04a20f7b16636fc5889201e73ac8625", None);
        assert_eq!(metainfo.clone().annouce(), "http://127.0.0.1:3000/announce");

        let client = reqwest::Client::new();
        let res = client
            .get(metainfo.annouce())
            .query(&query)
            .build()
            .unwrap();
        let mut pairs = res.url().query_pairs();

        assert_eq!(pairs.count(), 9);
        assert_eq!(
            pairs.next(),
            Some((
                Cow::Borrowed("info_hash"),
                Cow::Borrowed("%e0J%20%f7%b1f6%fcX%89%20%1es%ac%86%25")
            ))
        );
        assert_eq!(
            pairs.next(),
            Some((Cow::Borrowed("peer_id"), Cow::Borrowed("")))
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
            Some((Cow::Borrowed("left"), Cow::Borrowed("0")))
        );
        assert_eq!(
            pairs.next(),
            Some((Cow::Borrowed("compact"), Cow::Borrowed("true")))
        );
        assert_eq!(
            pairs.next(),
            Some((Cow::Borrowed("no_peer_id"), Cow::Borrowed("true")))
        );
        assert_eq!(
            pairs.next(),
            Some((Cow::Borrowed("numwant"), Cow::Borrowed("50")))
        );
    }
}
