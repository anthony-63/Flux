pub mod convert;
use std::path::PathBuf;
pub struct FluxMap {
    pub artist:String,
    pub song_name:String,
    pub mapper:String,
    pub map_data:Vec<u8>,
    pub music_data:Vec<u8>
}
impl FluxMap {
    pub fn save(self,path_to: PathBuf) {
        let mut flm_data = Vec::<u8>::with_capacity(self.music_data.len() + self.map_data.len() + self.artist.len() + self.song_name.len() + self.mapper.len() + 48); //48 is just to be safe (doesn't increase file size)
        flm_data.extend((self.artist.len() as u16).to_be_bytes());
        flm_data.extend(self.artist.as_bytes());
        flm_data.extend((self.song_name.len() as u16).to_be_bytes());
        flm_data.extend(self.song_name.as_bytes());
        flm_data.extend((self.mapper.len() as u16).to_be_bytes());
        flm_data.extend(self.mapper.as_bytes());
        flm_data.extend((self.map_data.len() as u32).to_be_bytes());
        flm_data.extend(self.map_data);
        flm_data.extend(self.music_data);
        std::fs::write(&path_to, flm_data).expect("Failed to write flm file. SHOULD NOT HAPPEN???");
    
    }
}