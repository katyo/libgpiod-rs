use crate::{Error, Result, Time};
use std::{io, mem::size_of_val, str, time};

#[inline(always)]
pub fn time_from_nanos(nanos: u64) -> Time {
    time::Duration::from_nanos(nanos)
}

#[inline(always)]
pub fn is_set<T>(flags: T, flag: T) -> bool
where
    T: core::ops::BitAnd<Output = T> + Eq + Copy,
{
    flags & flag == flag
}

#[inline(always)]
pub fn invalid_input(msg: &'static str) -> Error {
    Error::new(io::ErrorKind::InvalidInput, msg)
}

#[inline(always)]
pub fn invalid_data(msg: &'static str) -> Error {
    Error::new(io::ErrorKind::InvalidData, msg)
}

#[inline(always)]
pub fn check_size<T: ?Sized>(len: usize, val: &T) -> Result<()> {
    if len == size_of_val(val) {
        Ok(())
    } else {
        Err(invalid_data("Unexpected size"))
    }
}

#[inline(always)]
pub fn check_len<V, T: ?Sized>(slice: &[V], val: &T) -> Result<()> {
    if slice.len() <= size_of_val(val) {
        Ok(())
    } else {
        Err(invalid_input("Too many values"))
    }
}

#[inline(always)]
pub fn check_len_str<T: ?Sized>(slice: &str, val: &T) -> Result<()> {
    if slice.as_bytes().len() /* \0 */ < size_of_val(val) {
        Ok(())
    } else {
        Err(invalid_input("String too long"))
    }
}

#[inline(always)]
pub fn safe_set_str<const N: usize>(dst: &mut [u8; N], src: &str) -> Result<()> {
    check_len_str(src, dst)?;

    let src = src.as_bytes();
    dst[..src.len()].copy_from_slice(src);
    dst[src.len()] = 0;

    Ok(())
}

#[inline(always)]
pub fn safe_get_str(src: &[u8]) -> Result<&str> {
    Ok(str::from_utf8(src)
        .map_err(|_| invalid_data("Invalid UTF-8"))?
        .trim_end_matches('\0'))
}

/// This definition from libc
#[inline(always)]
pub fn major(dev: u64) -> u64 {
    let mut major = 0;
    major |= (dev & 0x00000000000fff00) >> 8;
    major |= (dev & 0xfffff00000000000) >> 32;
    major
}

/// This definition from libc
#[inline(always)]
pub fn minor(dev: u64) -> u64 {
    let mut minor = 0;
    minor |= dev & 0x00000000000000ff;
    minor |= (dev & 0x00000ffffff00000) >> 12;
    minor
}
