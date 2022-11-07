use std::{
    fmt, fs,
    io::{self, Read},
    path::Path,
    str::FromStr,
};

use crate::prelude::*;

use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt};
use uuid::Uuid;

mod box_type;
mod file_type_box;

use box_type::BoxType;

/// Read an entire file into a Vec<u8>
fn slurp<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, io::Error> {
    let mut buf = Vec::new();
    fs::File::open(path)?.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    let buf = slurp(path)?;

    let mut raw_boxes: Vec<Result<FullBox, InvalidFlag>> = Vec::new();
    let mut reader = util::SliceReader::new(&buf[..]);

    while !reader.is_empty() {
        raw_boxes.push(RawBox::parse(&mut reader).unwrap().try_into());
    }

    dbg!(raw_boxes);

    Ok(())
}

pub struct RawBox<'a> {
    typ: BoxType,
    payload: &'a [u8],
}

impl<'a> RawBox<'a> {
    pub fn parse(reader: &mut util::SliceReader<'a>) -> Result<RawBox<'a>, ()> {
        let raw_size = reader.read_u32::<BigEndian>().unwrap();
        let raw_type = reader.read_code().unwrap();

        let mut size = if raw_size == 1 {
            reader.read_u64::<BigEndian>().unwrap()
        } else {
            raw_size as u64
        };

        let typ = if raw_type == Code::from_str("uuid").unwrap() {
            Uuid::from_u128(reader.read_u128::<BigEndian>().unwrap()).into()
        } else {
            raw_type.into()
        };

        if size == 0 {
            size = reader.len() as u64;
        }

        let payload = reader.read_slice(size);

        Ok(RawBox { typ, payload })
    }
}

impl fmt::Debug for RawBox<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l = self.payload.len();
        f.debug_struct("Box")
            .field("type", &self.typ)
            .field(
                "payload",
                &format!(
                    "[{:02x?}{:02x?}{:02x?}{:02x?}..{:02x?}{:02x?}{:02x?}{:02x?}]",
                    self.payload[0],
                    self.payload[1],
                    self.payload[2],
                    self.payload[3],
                    self.payload[l - 4],
                    self.payload[l - 3],
                    self.payload[l - 2],
                    self.payload[l - 1]
                ),
            )
            .field("payload_len", &l)
            .finish()
    }
}

bitflags! {
    struct FullBoxFlags: u32 {
        const A = 0b1;
    }
}

pub struct FullBox<'a> {
    typ: BoxType,
    version: u8,
    flags: FullBoxFlags,
    payload: &'a [u8],
}

/// Invalid flag error
// TODO impl debug
#[derive(Debug)]
pub struct InvalidFlag(u32);

impl<'a> TryFrom<RawBox<'a>> for FullBox<'a> {
    type Error = InvalidFlag;
    fn try_from(value: RawBox<'a>) -> Result<Self, Self::Error> {
        let mut reader = util::SliceReader::new(value.payload);

        let raw_ext = reader.read_u32::<BigEndian>().unwrap();

        let version = (raw_ext >> (8 * 3)) as u8;
        let raw_flag = raw_ext & 0xff_ff_ff;
        let flags = FullBoxFlags::from_bits(raw_flag).ok_or(InvalidFlag(raw_flag))?;
        let payload = reader.into_inner();

        Ok(FullBox {
            typ: value.typ,
            version,
            flags,
            payload,
        })
    }
}

impl fmt::Debug for FullBox<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l = self.payload.len();
        f.debug_struct("FullBox")
            .field("type", &self.typ)
            .field(
                "payload",
                &format!(
                    "[{:02x?}{:02x?}{:02x?}{:02x?}..{:02x?}{:02x?}{:02x?}{:02x?}]",
                    self.payload[0],
                    self.payload[1],
                    self.payload[2],
                    self.payload[3],
                    self.payload[l - 4],
                    self.payload[l - 3],
                    self.payload[l - 2],
                    self.payload[l - 1]
                ),
            )
            .field("payload_len", &l)
            .field("version", &self.version)
            .field("flags", &self.flags)
            .finish()
    }
} /*
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
