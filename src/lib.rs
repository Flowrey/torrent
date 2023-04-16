pub mod metainfo;
pub mod tracker;

#[cfg(test)]
mod tests {
    use super::metainfo::{Info, Metainfo};

    #[test]
    fn build_metainfo() {
        let info = Info::new_single_file("e04a20f7b16636fc5889201e73ac8625", "debian.iso", 100);
        Metainfo::new(info, "http://localhost:8080");
    }
}
