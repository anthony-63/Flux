#[cfg(test)]
mod tests {
    use crate::FluxMap;


    #[test]
    fn parse_file() {
        let path = std::path::PathBuf::from("./data/maps/ss_archive_akira_complex_-_ether_strike.flux");
        let map = FluxMap::open(path).unwrap();
        println!("{:?}", map);
    }

}