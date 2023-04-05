use std::io::{BufReader, Cursor, BufRead, Read, Seek, SeekFrom};

use binrw::BinReaderExt;
use thiserror::Error;

use crate::FluxMap;

pub struct SSPM1 {
    pub music_data : Vec<u8>,
    pub map_data : Vec<SSPM1Note>,
    pub id : String,
    pub name : String,
    pub creator : String,
}
pub struct SSPM1NoteF {
    pub time : u32,
    pub x : f32,
    pub y : f32,
}
pub struct SSPM1Note8 {
    pub time : u32,
    pub x : u8,
    pub y : u8,
}
pub enum SSPM1Note {
    Float(SSPM1NoteF),
    Int(SSPM1Note8),
}
impl SSPM1Note {
    pub fn time(&self) -> u32 {
        match self {
            SSPM1Note::Float(x) => x.time,
            SSPM1Note::Int(x) => x.time,
        }
    }
}
#[derive(Debug,Error)]
pub enum MapParseErrorV1 {
    #[error("bad format pos: {0}")]
    BadFormat(u64),
    #[error("map has no audio")]
    NoAudio
}

impl TryFrom<Vec<u8>> for SSPM1 {
    type Error = MapParseErrorV1;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        SSPM1::try_from(Cursor::new(data.as_slice()))
    }
}
impl TryFrom<Cursor<&[u8]>> for SSPM1 {
    type Error = MapParseErrorV1;
    fn try_from(mut data: Cursor<&[u8]>) -> Result<Self, Self::Error> {
        let mut mid = String::new();
        let mut mname = String::new();
        let mut mcreator = String::new();
        let offset = {
            
            let mut r =BufReader::new(&mut data);
            r.read_line(&mut mid).or(Err(MapParseErrorV1::BadFormat(r.stream_position().unwrap())))?;
            r.read_line(&mut mname).or(Err(MapParseErrorV1::BadFormat(r.stream_position().unwrap())))?;
            r.read_line(&mut mcreator).or(Err(MapParseErrorV1::BadFormat(r.stream_position().unwrap())))?;
        
            let pos = r.stream_position().or(Err(MapParseErrorV1::BadFormat(r.stream_position().unwrap())))?;
            pos
    
        };
        let mut r = data;
        r.set_position(offset);
        let _last_ms : u32 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
        let note_count : u32 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
        let _diff :u8 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
        let img_type : u8 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
        match img_type {
            2 => {
                let len : u64 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
                r.seek(SeekFrom::Current(len as i64)).or(Err(MapParseErrorV1::BadFormat(r.position())))?;
            }
            1 => {
                let _height : u16 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
                let _width : u16 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
                let _mipmaps : u8 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
                let _format : u8 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
                let len : u64 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
                r.seek(SeekFrom::Current(len as i64)).or(Err(MapParseErrorV1::BadFormat(r.position())))?;
            }
            _ => {}
        }
        let has_audio :u8 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
        if has_audio != 1 {
            return Err(MapParseErrorV1::NoAudio); // no audio
        }
        let music_length : u64 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
        let mut music_d = vec![0;music_length as usize];
        r.read_exact(&mut music_d).or(Err(MapParseErrorV1::BadFormat(r.position())))?;
        let mut notes : Vec<SSPM1Note> = Vec::with_capacity(note_count as usize);
        for _ in 0..note_count {
            let time : u32 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
            let ntype : u8 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
            notes.push(match ntype {
                1 => {
                    let x : f32 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
                    let y : f32 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
                    SSPM1Note::Float(SSPM1NoteF {
                        time,
                        x,
                        y,
                    })
                }
                _=> {
                    let x : u8 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
                    let y : u8 = r.read_le().or(Err(MapParseErrorV1::BadFormat(r.position())))?;
                    SSPM1Note::Int(SSPM1Note8 {
                        time,
                        x : x,
                        y : y,
                    })
                }
            });
        }
        notes.sort_by(|x,y| x.time().cmp(&y.time()));
        
        
        Ok(Self {
            map_data : notes,
            music_data : music_d,
            id : mid,
            name : mname,
            creator : mcreator,
        })
    }
}

impl Into<FluxMap> for SSPM1 {
    fn into(self) -> FluxMap {
        let mut normalise_notes = String::new();
        normalise_notes.push('0');
        for note in self.map_data {
            normalise_notes.push(',');
            match note {
                SSPM1Note::Float(x) => {
                    normalise_notes.push_str(&format!("{}|{}|{}",x.x,x.y,x.time));
                }
                SSPM1Note::Int(x) => {
                    normalise_notes.push_str(&format!("{}|{}|{}",x.x,x.y,x.time));
                }
            }
        };
        FluxMap {
            artist : "<unknown>".to_string(),
            mapper : self.creator,
            song_name : self.name,
            music_data : self.music_data,
            map_data : normalise_notes.as_bytes().to_vec(),
        }



    }
}