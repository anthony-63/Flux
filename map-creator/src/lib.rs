pub mod convert;
use std::{path::PathBuf, io::Cursor};

use binrw::BinWriterExt;
use convert::sspm::SSPM;
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
        let mut w = Cursor::new(&mut flm_data);
        w.write_be(&(self.artist.len() as u16)).unwrap();
        w.write_be(&self.artist.as_bytes()).unwrap();
        w.write_be(&(self.song_name.len() as u16)).unwrap();
        w.write_be(&self.song_name.as_bytes()).unwrap();
        w.write_be(&(self.mapper.len() as u16)).unwrap();
        w.write_be(&self.mapper.as_bytes()).unwrap();
        w.write_be(&(self.map_data.len() as u32)).unwrap();
        w.write_be(&self.map_data).unwrap();
        w.write_be(&self.music_data).unwrap();
        std::fs::write(&path_to, flm_data).expect("Failed to write flm file. SHOULD NOT HAPPEN???");
    
    }
}