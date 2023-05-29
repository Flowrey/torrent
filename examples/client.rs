use bittorent::{metainfo::Metainfo, tracker::{Request, self}};
use reqwest;

const DEBIAN_URL: &'static str = "https://cdimage.debian.org/debian-cd/current/amd64/bt-cd/debian-11.7.0-amd64-netinst.iso.torrent";

#[tokio::main]
async fn main() {
    let torrent = reqwest::get(DEBIAN_URL)
        .await
        .expect("Failed to send request")
        .bytes()
        .await
        .expect("Failed to get torrent");

    let metainfo = Metainfo::try_from_bytes(&torrent)
        .expect("Failed to get metainfo from bytes");

    let info_hash = hex::encode(metainfo
        .clone()
        .info()
        .pieces()
    );

    let query = Request::new(&info_hash, None);
    let client = reqwest::Client::new();
    let request = client
        .get(metainfo.annouce())
        .query(&query)
        .build()
        .unwrap();

    let res = client
        .execute(request)
        .await
        .unwrap()
        .json::<tracker::Response>()
        .await
        .unwrap();

    println!("{:?}", res);
}
