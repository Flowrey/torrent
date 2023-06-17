mod error;
pub mod metainfo;
pub use metainfo::MetaInfo;

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    const DEBIAN_URL: &'static str = "https://cdimage.debian.org/debian-cd/current/amd64/bt-cd/debian-12.0.0-amd64-netinst.iso.torrent";

    #[test]
    fn parse_torrent_file() {
        let metainfo = MetaInfo::from_url(DEBIAN_URL).unwrap();
        let info = metainfo.info();

        assert_eq!(
            metainfo.announce(),
            "http://bttracker.debian.org:6969/announce"
        );
        assert_eq!(info.name(), "debian-12.0.0-amd64-netinst.iso");
        assert_eq!(
            info.hash()[..],
            hex!("b851474b74f65cd19f981c723590e3e520242b97")
        );
    }
}
