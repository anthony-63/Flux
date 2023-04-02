pub const SUPER_SPECIAL_SEP: &str = "⁜";

pub struct FluxMap {
    pub artist: String,
    pub song_name: String,
    pub mapper: String,
    pub map_data: String,
    pub mp3_data: Vec<u8>,
}

impl FluxMap {
    pub fn empty() -> Self {
        Self {
            artist: String::from(""),
            song_name: String::from(""),
            mapper: String::from(""),
            map_data: String::from(""),
            mp3_data: vec![],
        }
    }
}

pub struct FluxMaploader;

impl FluxMaploader {
    pub fn load_map(path: String) -> FluxMap {
        let mut map = FluxMap::empty();
        
        let flm_data = match std::str::from_utf8(&std::fs::read(&path).expect("Failed to read map file. SHOULD NOT HAPPEN???")) {
            Ok(v) => v.to_owned(),
            Err(e) => panic!("Invalid utf-8 sequence: {}", e),
        };

        let sections: Vec<_> = flm_data.split(SUPER_SPECIAL_SEP).collect();
        let meta = sections[0].to_string();
        map.map_data = sections[1].to_string();
        map.mp3_data = sections[2].as_bytes().to_vec();

        let meta_sections: Vec<_> = meta.split(",").collect();
        map.artist = meta_sections[0].to_string();
        map.song_name = meta_sections[1].to_string();
        map.mapper = meta_sections[2].to_string();

        println!("Map metadata: {}", meta);
        map
        
    }
}