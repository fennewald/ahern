use std::{
    fmt,
    io::{self, Cursor, Read},
    str::FromStr,
};

use crate::prelude::*;

use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt};
use uuid::Uuid;

mod box_type;

use box_type::BoxType;

pub struct RawBox {
    typ: BoxType,
    payload: Vec<u8>,
}

impl RawBox {
    pub fn read<R: Read>(fh: &mut R) -> Result<RawBox, io::Error> {
        let raw_size = fh.read_u32::<BigEndian>()?;
        let raw_type = fh.read_code()?;

        let size = if raw_size == 1 {
            fh.read_u64::<BigEndian>()?
        } else {
            raw_size as u64
        };

        let typ = if raw_type == Code::from_str("uuid").unwrap() {
            Uuid::from_u128(fh.read_u128::<BigEndian>()?).into()
        } else {
            raw_type.into()
        };

        let payload = if size == 0 {
            let mut buf = Vec::new();
            fh.read_to_end(&mut buf)?;
            buf
        } else {
            let mut buf = vec![0; size as usize];
            fh.read_exact(&mut buf)?;
            buf
        };

        Ok(RawBox { typ, payload })
    }
}

impl fmt::Debug for RawBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l = self.payload.len();
        write!(
            f,
            "Box {{ typ: {:?}, payload: [{:02x?}{:02x?}{:02x?}{:02x}..{:02x?}{:02x?}{:02x?}{:02x}] (length: {}) }}",
            self.typ,
            self.payload[0], self.payload[1], self.payload[2], self.payload[3],
            self.payload[l-4], self.payload[l-3], self.payload[l-2], self.payload[l-1],
            l
        )
    }
}

bitflags! {
    struct FullBoxFlags: u32 {
        const A = 0b1;
    }
}

pub struct FullBox {
    typ: BoxType,
    version: u8,
    flags: FullBoxFlags,
    payload: Vec<u8>,
}

/*
impl From<RawBox> for FullBox {
    fn from(value: RawBox) -> Self {
        let buf: [u8; 4] = [0; 4];

        let

        FullBox {
            value.typ,
        }
    }
}
*/

/*
trait BoxImpl {
    const CODE: BoxType;

    fn parse(raw: Vec<u8>) -> Self;
}

pub struct FileTypeBox {
    major_brand: u32,
    minor_version: u32,
    compatible_brands: Vec<u32>,
}

impl BoxImpl for FileTypeBox {
    const CODE: BoxType = BoxType::from_code("ftyp");

    fn parse(raw: Vec<u8>) -> Self {
        let mut c = Cursor::new(raw);
        let major_brand = c.read_u32::<BigEndian>().unwrap();
        let minor_version = c.read_u32::<BigEndian>().unwrap();

        FileTypeBox {
            major_brand,
            minor_version,
        todo!()
    }
}
*/
