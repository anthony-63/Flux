pub mod tests;
pub mod convert;
use std::{path::PathBuf, io::Cursor, collections::HashMap};

use binrw::{BinWriterExt, BinReaderExt, binrw, VecArgs};
use thiserror::Error;
#[derive(Debug)]
pub struct FluxMap {
    pub version:u8,
    pub meta : HashMap<String,Vec<u8>>,
    pub difficulties:HashMap<String,Vec<FluxNote>>,
    pub music_data:Vec<u8>,
    pub image_data:Option<Vec<u8>>,
}
#[derive(Debug)]
pub struct FluxNote {
    pub time:u32,
    pub x:f32,
    pub y:f32
}
impl FluxNote {
    pub fn new(time:u32,x:f32,y:f32) -> Self {
        Self {
            time,
            x,
            y,
        }
    }
}
const FLUX_SIG : [u8;4] = [b'F',b'L',b'U',b'X'];


#[binrw]
#[br(big)]
pub(crate)struct SizedVec {
    len:u32,
    #[br(count=len)]
    data:Vec<u8>,
}
#[binrw]
#[br(big)]
pub(crate)struct SizedString {
    len:u16,
    #[br(count=len)]
    data:Vec<u8>,
}
impl ToString for SizedString {
    fn to_string(&self) -> String {
        String::from_utf8(self.data.clone()).unwrap()
    }
}

impl FluxMap {
    pub fn new() -> Self {
        Self {
            version:1,
            meta:HashMap::new(),
            difficulties:HashMap::new(),
            music_data:Vec::new(),
            image_data:None,
        }
    }
    pub fn add_metadata(&mut self,key:String,value:Vec<u8>) {
        self.meta.insert(key,value);
    }
    pub fn add_difficulty(&mut self,key:String,value:Vec<FluxNote>) {
        self.difficulties.insert(key,value);
    }
    pub fn add_music(&mut self,data:Vec<u8>) {
        self.music_data = data;
    }
    pub fn add_image(&mut self,data:Vec<u8>) {
        self.image_data = Some(data);
    }
    pub fn open(path_from: PathBuf) -> Result<Self,FluxMapError> {
        let all_data = std::fs::read(path_from).unwrap();
        Self::parse_data(&all_data)
    }
    pub fn parse_data(data:&[u8]) -> Result<Self,FluxMapError> {
        let mut r = Cursor::new(data);
        let sig = r.read_be::<[u8;4]>().or_else(|_| Err(FluxMapError::BadFormat(FluxBadFormatType::BadSignature)))?;
        if sig != FLUX_SIG {
            return Err(FluxMapError::BadFormat(FluxBadFormatType::BadSignature));
        }
        let version = r.read_be::<u8>().or_else(|_| Err(FluxMapError::BadFormat(FluxBadFormatType::BadVersion)))?;
        match version {
            1 => {
                let mut meta = HashMap::new();
                let meta_count = r.read_be::<u16>().or_else(|_| Err(FluxMapError::BadFormat(FluxBadFormatType::BadMetadata)))?;
                for _ in 0..meta_count {
                    let key : SizedString = r.read_be().or_else(|_| Err(FluxMapError::BadFormat(FluxBadFormatType::BadMetadata)))?;
                    let value : SizedVec = r.read_be().or_else(|_| Err(FluxMapError::BadFormat(FluxBadFormatType::BadMetadata)))?;
                    meta.insert(key.to_string(),value.data);
                }
                let mut difficulties = HashMap::new();
                let difficulty_count = r.read_be::<u16>().or_else(|_| Err(FluxMapError::BadFormat(FluxBadFormatType::BadDifficulty)))?;
                for _ in 0..difficulty_count {
                    let key : SizedString = r.read_be().or_else(|_| Err(FluxMapError::BadFormat(FluxBadFormatType::BadDifficulty)))?;
                    let note_count = r.read_be::<u64>().or_else(|_| Err(FluxMapError::BadFormat(FluxBadFormatType::BadDifficulty)))?;
                    let mut notes = Vec::with_capacity(note_count as usize);
                    for _ in 0..note_count {
                        let time = r.read_be::<u32>().or_else(|_| Err(FluxMapError::BadFormat(FluxBadFormatType::BadDifficulty)))?;
                        let x = r.read_be::<f32>().or_else(|_| Err(FluxMapError::BadFormat(FluxBadFormatType::BadDifficulty)))?;
                        let y = r.read_be::<f32>().or_else(|_| Err(FluxMapError::BadFormat(FluxBadFormatType::BadDifficulty)))?;
                        let note = FluxNote::new(time,x,y);
                        notes.push(note);
                    }
                    difficulties.insert(key.to_string(),notes);
                }
                let image_len = r.read_be::<u32>().or_else(|_| Err(FluxMapError::BadFormat(FluxBadFormatType::BadDifficulty)))?;
                let image_data = if image_len == 0 {
                    None
                } else {
                    Some(r.read_be_args(VecArgs::builder().count(image_len as usize).finalize()).unwrap())
                };
                let music = r.read_be::<SizedVec>().or_else(|_| Err(FluxMapError::BadFormat(FluxBadFormatType::BadDifficulty)))?;
                Ok(Self {
                    version,
                    meta,
                    difficulties,
                    music_data: music.data,
                    image_data,
                })
            },
            _ => Err(FluxMapError::BadFormat(FluxBadFormatType::BadVersion))
        }
    }
    pub fn save(self,path_to: PathBuf) {
        let mut flm_data = Vec::<u8>::with_capacity(
            self.music_data.len() 
            + self.difficulties.values().fold(0, |x,f| x+ f.len()) 
            + self.meta.len()
            + 48 //48 is just to be safe (doesn't increase file size)
        ); 
        let mut w = Cursor::new(&mut flm_data);
        w.write_be(&FLUX_SIG).unwrap();
        w.write_be(&self.version).unwrap();
        // meta key count
        w.write_be(&(self.meta.len() as u16)).unwrap();
        // write metadata
        for (k,v) in self.meta {
            w.write_be(&(k.len() as u16)).unwrap();
            w.write_be(&k.as_bytes()).unwrap();
            w.write_be(&(v.len() as u32)).unwrap();
            w.write_be(&v).unwrap();
        }
        // difficulty count
        w.write_be(&(self.difficulties.len() as u16)).unwrap();
        // write difficulties
        for (k,v) in self.difficulties {
            w.write_be(&(k.len() as u16)).unwrap();
            w.write_be(&k.as_bytes()).unwrap();
            w.write_be(&(v.len() as u64)).unwrap();
            for note in v {
                w.write_be(&note.time).unwrap();
                w.write_be(&note.x).unwrap();
                w.write_be(&note.y).unwrap();
            }
        }
        // write image data        
        if let Some(image_data) = self.image_data.as_ref() {
            w.write_be(&(image_data.len() as u32)).unwrap();
            w.write_be(image_data).unwrap();
        } else {
            w.write_be(&(0 as u32)).unwrap();
        }
        // write music data
        w.write_be(&(self.music_data.len() as u32)).unwrap();
        w.write_be(&self.music_data).unwrap();
        std::fs::write(&path_to, flm_data).expect("Failed to write flm file. SHOULD NOT HAPPEN???");
    
    }
    pub fn convert_ss_to_flux(ssmap:&Vec<u8>) -> Vec<FluxNote> {
        let as_str = std::str::from_utf8(&ssmap).unwrap();
        let itr = as_str.split(",").skip(1);
        let mut notes : Vec<FluxNote> = Vec::new();
        for enotes in itr {
            let note = enotes.split("|").collect::<Vec<&str>>();
            let x = note[0].parse::<f32>().unwrap();
            let y = note[1].parse::<f32>().unwrap();
            let time = note[2].parse::<u32>().unwrap();
            notes.push(FluxNote::new(time,x,y));
        }
        notes
    }
}
impl TryFrom<&[u8]> for FluxMap {
    type Error = FluxMapError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Self::parse_data(data)
    }
}


#[derive(Debug,Error)]
pub enum FluxMapError {
    #[error("Bad Format {0}")]
    BadFormat(FluxBadFormatType),
}
#[derive(Debug,Error)]
pub enum FluxBadFormatType {
    #[error("Bad Signature")]
    BadSignature,
    #[error("Bad Version")]
    BadVersion,
    #[error("Bad Metadata")]
    BadMetadata,
    #[error("Bad Difficulty")]
    BadDifficulty,
    #[error("Bad Image")]
    BadImage,
    #[error("Bad Music")]
    BadMusic,
}