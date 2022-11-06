use std::{fmt, io, str::FromStr};

use byteorder::ReadBytesExt;
use smallstr::SmallString;

/// A 4-digit character code
#[repr(transparent)]
#[derive(PartialEq)]
pub struct Code(u32);

impl Code {
    pub fn as_str(&self) -> SmallString<[u8; 4]> {
        SmallString::from_buf(self.0.to_be_bytes()).unwrap()
    }
}

impl FromStr for Code {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf: [u8; 4] = s.as_bytes().try_into().unwrap();
        Ok(buf.into())
    }
}

impl Into<u32> for Code {
    fn into(self) -> u32 {
        self.0
    }
}

impl From<&[u8; 4]> for Code {
    fn from(value: &[u8; 4]) -> Self {
        (*value).into()
    }
}

impl From<[u8; 4]> for Code {
    fn from(value: [u8; 4]) -> Self {
        Code(u32::from_be_bytes(value))
    }
}

impl From<u32> for Code {
    fn from(value: u32) -> Self {
        Code(value)
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

pub trait ReadCode: ReadBytesExt {
    fn read_code(&mut self) -> io::Result<Code> {
        self.read_u32::<byteorder::BigEndian>().map(|n| n.into())
    }
}

impl<R: ReadBytesExt + ?Sized> ReadCode for R {}
