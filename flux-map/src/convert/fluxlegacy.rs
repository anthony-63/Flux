use std::io::{Cursor, Read};

use binrw::BinReaderExt;
use thiserror::Error;

use crate::{SizedString, SizedVec, FluxMap};

pub struct FluxLegacy {
    pub artist: String,
    pub song_name: String,
    pub mapper: String,
    pub map_data: String,
    pub mp3_data: Vec<u8>,
}
impl TryFrom<Cursor<&[u8]>> for FluxLegacy {
    type Error = FluxLegacyError;
    fn try_from(mut datac: Cursor<&[u8]>) -> Result<Self, Self::Error> {
        let artist :SizedString = datac.read_be().or(Err(FluxLegacyError::BadFormat(datac.position())))?;
        let song_name :SizedString = datac.read_be().or(Err(FluxLegacyError::BadFormat(datac.position())))?;
        let mapper :SizedString = datac.read_be().or(Err(FluxLegacyError::BadFormat(datac.position())))?;
        let map_data :SizedVec = datac.read_be().or(Err(FluxLegacyError::BadFormat(datac.position())))?;
        let mut mp3_data = Vec::new();
        datac.read_to_end(&mut mp3_data).or(Err(FluxLegacyError::BadFormat(datac.position())))?;
        let map_data_str = String::from_utf8(map_data.data).or(Err(FluxLegacyError::BadFormat(datac.position())))?;
        Ok(FluxLegacy {
            artist: artist.to_string(),
            song_name: song_name.to_string(),
            mapper: mapper.to_string(),
            map_data: map_data_str,
            mp3_data: mp3_data,
        })
    }
}
impl TryFrom<&[u8]> for FluxLegacy {
    type Error = FluxLegacyError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let cur = Cursor::new(data);
        Self::try_from(cur)
    }
}
#[derive(Debug,Error)]
pub enum FluxLegacyError {
    #[error("bad format pos: {0}")]
    BadFormat(u64),
}
impl Into<FluxMap> for FluxLegacy {
    fn into(self) -> FluxMap {
        let mut m = FluxMap::new();
        m.add_metadata("mapper".to_string(), self.mapper.as_bytes().to_vec());
        m.add_metadata("artist".to_string(), self.artist.as_bytes().to_vec());
        m.add_metadata("song_name".to_string(), self.song_name.as_bytes().to_vec());
        m.add_difficulty("default".to_string(), FluxMap::convert_ss_to_flux(&self.map_data.as_bytes().to_vec()));
        m.add_music(self.mp3_data);

        m
    }
}