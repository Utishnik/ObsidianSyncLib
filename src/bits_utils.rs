#[inline]
pub fn size_bytes<T>() -> usize {
    let size_bytes: usize = std::mem::size_of::<T>();
    size_bytes
}
#[inline]
pub fn size_bits<T>() -> usize {
    let size_bits: usize = size_bytes::<T>() * 8;
    size_bits
}
