pub fn urlencode(in_str: &[u8]) -> String {
    let mut escaped_info_hash = String::new();
    for byte in in_str {
        if byte.is_ascii_alphanumeric() || [b'.', b'-', b'_', b'~', b'*'].contains(byte) {
            escaped_info_hash.push(*byte as char);
        } else {
            let str = format!("%{:02x}", byte);
            escaped_info_hash.push_str(&str);
        };
    }
    escaped_info_hash
}

#[cfg(test)]
mod test {
    use hex_literal::hex;

    use super::urlencode;

    #[test]
    fn test_urlencode() {
        let got = urlencode(&hex!("6f84758b0ddd8dc05840bf932a77935d8b5b8b93"));
        let expected = "o%84u%8b%0d%dd%8d%c0X%40%bf%93*w%93%5d%8b%5b%8b%93";
        assert_eq!(got, expected)
    }
}