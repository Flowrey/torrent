pub mod metainfo;
pub mod tracker;

#[cfg(test)]
mod tests {
    use super::metainfo::{Info, Metainfo};

    #[test]
    fn build_metainfo() {
        Metainfo::new(
            Info::new_single_file("e04a20f7b16636fc5889201e73ac8625", "debian.iso", 100),
            "http://localhost:8080",
        );
    }
}
