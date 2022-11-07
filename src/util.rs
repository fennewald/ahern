use std::io;

/// A reader that wraps a borrowed slice, and ultimately can be turned into the
/// remaining slice
#[repr(transparent)]
pub struct SliceReader<'a>(&'a [u8]);

impl<'a> SliceReader<'a> {
    pub fn new(slice: &'a [u8]) -> SliceReader<'a> {
        SliceReader(slice)
    }

    /// Read a slice of the provided length
    pub fn read_slice(&mut self, len: u64) -> &'a [u8] {
        let (l, r) = self.0.split_at(len as usize);
        self.0 = r;
        l
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return an owned reference to the slice's internal slice
    pub fn into_inner(self) -> &'a [u8] {
        self.0
    }
}

impl<'a> io::Read for SliceReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = std::cmp::min(buf.len(), self.0.len()) as u64;
        let res = self.read_slice(len);
        buf.copy_from_slice(res);
        Ok(len as usize)
    }
}
