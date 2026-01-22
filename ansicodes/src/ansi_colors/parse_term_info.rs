use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

pub fn read_le_u16(r: &mut dyn io::Read) -> io::Result<u32> {
    let mut buf: [u8; 2] = [0; 2];
    r.read_exact(&mut buf)
        .map(|()| u32::from(u16::from_le_bytes(buf)))
}

pub fn read_le_u32(r: &mut dyn io::Read) -> io::Result<u32> {
    let mut buf: [u8; 4] = [0; 4];
    r.read_exact(&mut buf).map(|()| u32::from_le_bytes(buf))
}

fn read_byte(r: &mut dyn io::Read) -> io::Result<u8> {
    // We allow this because it's up to the caller to pass us a buffered reader when necessary
    // (which is exactly what we do in this library).
    #[allow(clippy::unbuffered_bytes)]
    match r.bytes().next() {
        Some(s) => s,
        None => Err(io::Error::new(io::ErrorKind::Other, "end of file")),
    }
}
