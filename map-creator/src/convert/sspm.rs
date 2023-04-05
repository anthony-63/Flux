use std::io::Cursor;

use binrw::BinReaderExt;
use thiserror::Error;

use crate::FluxMap;

use super::sspmv1::{SSPM1, MapParseErrorV1};

pub enum SSPM {
    V1(SSPM1),
    V2
}
impl TryFrom<Vec<u8>> for SSPM {
    type Error = MapParseError;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        let mut cur = Cursor::new(data.as_slice());
        let sig : [u8;4] = cur.read_le().or(Err(MapParseError::BadFormat(cur.position())))?;
        if sig != [0x53,0x53,0x2b,0x6d] {
            println!("sig: {:?}",sig);
            return Err(MapParseError::UnknownSig(sig.to_vec()));
        }
        let version : u16 = cur.read_le().or(Err(MapParseError::BadFormat(cur.position())))?;
        match version {
            1 => {
                let sspm1 = SSPM1::try_from(cur).map_err(|e| MapParseError::V1(e))?;
                Ok(SSPM::V1(sspm1))
            },
            2 => {
                Ok(SSPM::V2)
            },
            _ => Err(MapParseError::UnknownVer(version)),
        }
    }
}
impl Into<FluxMap> for SSPM {
    fn into(self) -> FluxMap {
        match self {
            SSPM::V1(x) => x.into(),
            SSPM::V2 => panic!("not impl."),
        }
    }
}
#[derive(Debug,Error)]
pub enum MapParseError {
    #[error("{0}")]
    V1(MapParseErrorV1),
    #[error("hu")]
    V2,
    #[error("Unknown signature {0:x?}")]
    UnknownSig(Vec<u8>),
    #[error("Unknown version '{0}'")]
    UnknownVer(u16),
    #[error("bad format pos: {0}")]
    BadFormat(u64),
}