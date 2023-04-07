#[cfg(test)]
mod tests {
    use crate::FluxMap;


    #[test]
    fn parse_file() {
        println!("cwd: {:?}", std::env::current_dir().unwrap());
        let path = std::path::PathBuf::from("../data/maps/ss_archive_akira_complex_-_ether_strike.flux");
        let map = FluxMap::open(path).unwrap();
        assert!(map.version == 1);
        assert!(map.meta.len() == 3);
        assert!(map.difficulties.len() == 1);
    }

}