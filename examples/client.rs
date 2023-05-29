use bittorent::metainfo::Metainfo;
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

    let metainfo = Metainfo::try_from_bytes(&torrent).expect("Failed to get metainfo from bytes");
    println!("{:?}", metainfo)
}
