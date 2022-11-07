use crate::prelude::*;

#[derive(Debug)]
pub struct FileTypeBox {
    major_brand: Code,
    minor_version: Code,
    compatible_brands: Vec<u32>,
}
