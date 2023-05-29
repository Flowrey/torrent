const HANDSHAKE_LEN: usize = 68;

pub struct Handshake([u8; HANDSHAKE_LEN]);

impl Handshake {
    pub fn new(info_hash: [u8; 20], peer_id: &str) -> Handshake {
        let mut buff = [0u8; HANDSHAKE_LEN];
        buff[0] = 19;
        buff[1..20].copy_from_slice(b"BitTorrent protocol");
        buff[20..28].copy_from_slice(&[0u8; 8]);
        buff[28..48].copy_from_slice(&info_hash);
        buff[48..68].copy_from_slice(peer_id.as_bytes());
        Handshake(buff)
    }

    pub fn pstrlen(&self) -> u8 {
        self.0[0]
    }

    pub fn pstr(&self) -> &[u8] {
        &self.0[1..20]
    }

    pub fn info_hash(&self) -> &[u8] {
        &self.0[28..48]
    }

    pub fn peer_id(&self) -> &[u8] {
        &self.0[48..HANDSHAKE_LEN]
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Handshake;
    use hex_literal::hex;

    #[test]
    fn test_handshake() {
        let hash = hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
        let handshake = Handshake::new(hash.try_into().unwrap(), "--------------------");
        println!("{:?}", handshake.as_bytes())
    }
}
