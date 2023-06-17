mod error;
pub mod metainfo;
pub use metainfo::MetaInfo;

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

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
}
