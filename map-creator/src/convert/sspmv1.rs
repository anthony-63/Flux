use std::io::{BufReader, Cursor, BufRead, Read, Seek, SeekFrom};

use binrw::BinReaderExt;

use crate::FluxMap;

pub struct SSPM1 {
    music_data : Vec<u8>,
    map_data : Vec<SSPM1Note>,
    id : String,
    name : String,
    creator : String,
}
pub struct SSPM1NoteF {
    time : u32,
    x : f32,
    y : f32,
}
pub struct SSPM1Note8 {
    time : u32,
    x : f32,
    y : f32,
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

impl TryFrom<Vec<u8>> for SSPM1 {
    type Error = ();
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        let mut mid = String::new();
        let mut mname = String::new();
        let mut mcreator = String::new();
        let data = {
            let mut vd = vec![];
            let mut r =BufReader::new(Cursor::new(data));
            r.read_line(&mut mid);
            r.read_line(&mut mname);
            r.read_line(&mut mcreator);
        
            r.read_to_end(&mut vd);
            vd
    
        };
        let mut r = Cursor::new(data);
        let last_ms : u32 = r.read_le().or(Err(()))?;
        let note_count : u32 = r.read_le().or(Err(()))?;
        let mut diff :u8 = r.read_le().or(Err(()))?;
        let mut img_type : u8 = r.read_le().or(Err(()))?;
        match img_type {
            2 => {
                let mut len : u64 = r.read_le().or(Err(()))?;
                r.seek(SeekFrom::Current(len as i64)).or(Err(()))?;
            }
            _ => {}
        }
        let has_audio :u8 = r.read_le().or(Err(()))?;
        if has_audio != 1 {
            return Err(()); // no audio
        }
        let music_length : u64 = r.read_le().or(Err(()))?;
        let mut music_d = vec![0;music_length as usize];
        r.read_exact(&mut music_d).or(Err(()))?;
        let mut notes : Vec<SSPM1Note> = Vec::with_capacity(note_count as usize);
        for i in 0..note_count {
            let time : u32 = r.read_le().or(Err(()))?;
            let ntype : u8 = r.read_le().or(Err(()))?;
            notes.push(match ntype {
                1 => {
                    let x : f32 = r.read_le().or(Err(()))?;
                    let y : f32 = r.read_le().or(Err(()))?;
                    SSPM1Note::Float(SSPM1NoteF {
                        time,
                        x,
                        y,
                    })
                }
                _=> {
                    let x : u8 = r.read_le().or(Err(()))?;
                    let y : u8 = r.read_le().or(Err(()))?;
                    SSPM1Note::Int(SSPM1Note8 {
                        time,
                        x : x as f32,
                        y : y as f32,
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