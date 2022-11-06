use crate::prelude::*;

use std::fmt;

use uuid::Uuid;

const ISO_RESERVED: (u16, u16, [u8; 8]) = (0x0011, 0x0010, 0x8000_00AA00389B71u64.to_be_bytes());

#[derive(Copy, Clone)]
pub struct BoxType(Uuid);

impl BoxType {
    /// Returns if this type is an iso base type
    pub fn is_iso(&self) -> bool {
        let suffix = &self.0.as_u128().to_be_bytes()[4..];
        suffix[0..2] == ISO_RESERVED.0.to_be_bytes()
            && suffix[2..4] == ISO_RESERVED.1.to_be_bytes()
            && suffix[4..] == ISO_RESERVED.2
    }

    /// Returns the 32bit code for the type, represented as a u32
    pub fn code(&self) -> Code {
        if self.is_iso() {
            self.0.to_fields_le().0.swap_bytes().into()
        } else {
            b"uuid".into()
        }
    }

    /// Returns the uuid of the type
    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Debug for BoxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_iso() {
            write!(f, "{:?}", self.code().as_str())
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}

impl From<Code> for BoxType {
    fn from(value: Code) -> Self {
        BoxType(Uuid::from_fields(
            value.into(),
            ISO_RESERVED.0,
            ISO_RESERVED.1,
            &ISO_RESERVED.2,
        ))
    }
}

impl From<Uuid> for BoxType {
    fn from(value: Uuid) -> Self {
        BoxType(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::str::FromStr;

    #[test]
    fn from_code() {
        let code = Code::from_str("ftyp").unwrap();
        assert_eq!(code.into().code(), code)
    }

    #[test]
    fn suffixes() {
        assert!(uuid!("00000000-0011-0010-8000-00AA00389B71")
            .into()
            .is_iso());
    }
}
