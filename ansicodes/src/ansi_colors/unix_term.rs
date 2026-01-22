use core::ptr;
use core::{fmt::Display, mem, str};
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::os::fd::{AsRawFd, RawFd};
use libc::{STDIN_FILENO,STDOUT_FILENO,STDERR_FILENO};

#[derive(Clone, Debug)]
pub struct Term {
    pub(crate) is_msys_tty: bool,
    pub(crate) is_tty: u8,
}


/// possible stream sources
#[derive(Clone, Copy, Debug)]
pub enum Stream {
    Stdout,
    Stderr,
    Stdin,
}

#[cfg(all(unix, not(target_arch = "wasm32")))]
pub fn as_fd(stream: Stream) -> libc::c_int {
    extern crate libc;

    let fd: i32 = match stream {
        Stream::Stdout => libc::STDOUT_FILENO,
        Stream::Stderr => libc::STDERR_FILENO,
        Stream::Stdin => libc::STDIN_FILENO,
    };
    fd
}

#[inline]
pub fn get_tty_flag() -> u8
{
    let mut ret: u8=0;
    unsafe{
        let stdin_is_tty: bool = libc::isatty(STDIN_FILENO) == 1;
        let stdout_is_tty: bool = libc::isatty(STDOUT_FILENO) == 1;
        let stderr_is_tty: bool = libc::isatty(STDERR_FILENO) == 1;
        ret |= stdin_is_tty as u8;
        ret |= (stdout_is_tty as u8)<<1;
        ret |= (stderr_is_tty as u8)<<2;
    }
    ret
}

#[inline]
pub fn not_tty() -> bool{
    let res: u8 = get_tty_flag();
    res==0
}

pub fn is_a_color_terminal(out: &Term) -> bool {
    if !not_tty() {
        return false;
    }

    if env::var("NO_COLOR").is_ok() {
        return false;
    }

    match env::var("TERM") {
        Ok(term) => term != "dumb",
        Err(_) => false,
    }
}