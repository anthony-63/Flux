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
        
        let flm_data = std::fs::read(&path).expect("Failed to read map file. SHOULD NOT HAPPEN???");

        // you can probably use an iterator here, but honestly it would probably increase the complexity.
        // ^^ or you can just use another crate to do this for you.
        // you can also add additional error checking, but im too lazy.

        let mut index = 0;
        
        let artist_size = u16::from_be_bytes((&flm_data[index..index+2]).try_into().unwrap());
        index += 2;
        map.artist = String::from_utf8(flm_data[index..index+artist_size as usize].to_vec()).unwrap();
        index += artist_size as usize;

        let song_name_size = u16::from_be_bytes((&flm_data[index..index+2]).try_into().unwrap());
        index += 2;
        map.song_name = String::from_utf8(flm_data[index..index+song_name_size as usize].to_vec()).unwrap();
        index += song_name_size as usize;

        let mapper_size = u16::from_be_bytes((&flm_data[index..index+2]).try_into().unwrap());
        index += 2;
        map.mapper = String::from_utf8(flm_data[index..index+mapper_size as usize].to_vec()).unwrap();
        index += mapper_size as usize;

        let map_data_size = u32::from_be_bytes((&flm_data[index..index+4]).try_into().unwrap());
        index += 4;
        map.map_data = String::from_utf8(flm_data[index..index+map_data_size as usize].to_vec()).unwrap();
        index += map_data_size as usize;

        map.mp3_data = flm_data[index..].to_vec();

        println!("Map metadata: {},{},{}", map.artist, map.song_name, map.mapper);
        map
        
    }
}