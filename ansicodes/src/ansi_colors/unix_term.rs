use alloc::ffi::CString;
use core::ptr;
use core::{fmt::Display, mem, str};
use libc::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO};
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::os::fd::{AsRawFd, RawFd};

#[derive(Clone, Debug, Copy)]
pub struct Term {
    pub(crate) is_msys_tty: bool,
    pub(crate) is_tty: u8,
}

pub fn is_msys_tty(fd: libc::c_int) -> bool {
    unsafe {
        if not_tty_fd(fd) {
            return false;
        }

        // 2. Проверка переменных окружения
        let mut is_msys: bool = false;

        // Проверяем MSYSTEM (MSYS2)
        if let Ok(system) = std::env::var("MSYSTEM") {
            is_msys = system.starts_with("MINGW") || system == "MSYS" || system == "CYGWIN";
        }

        // Проверяем TERM
        if let Ok(term) = std::env::var("TERM") {
            is_msys = is_msys || term.contains("cygwin");
        }

        // Проверяем наличие cygwin/msys в пути
        if let Ok(path) = std::env::var("PATH") {
            is_msys = is_msys
                || path.contains("cygwin")
                || path.contains("msys2")
                || path.contains("MSYS2");
        }

        // 3. Проверка имени терминала через ttyname
        let mut buf: [i8; 1024] = [0i8; 1024];
        if libc::ttyname_r(fd, buf.as_mut_ptr() as *mut libc::c_char, buf.len()) == 0 {
            let tty_name: String = std::ffi::CStr::from_ptr(buf.as_ptr())
                .to_string_lossy()
                .to_string();
            is_msys = is_msys || tty_name.contains("msys") || tty_name.contains("cygwin");
        }

        is_msys
    }
}

impl Term {
    fn set_tty_flag(&mut self, flag: u8) {
        self.is_tty = flag;
    }
    fn get_tty_flag(&self) -> u8 {
        self.is_tty
    }
}

impl Term {
    fn set_msys_tty(&mut self, val: bool) {
        self.is_msys_tty = val;
    }
    fn get_msys_tty(&self) -> bool {
        self.is_msys_tty
    }
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
pub fn get_tty_flag() -> u8 {
    let mut ret: u8 = 0;
    unsafe {
        let stdin_is_tty: bool = libc::isatty(STDIN_FILENO) == 1;
        let stdout_is_tty: bool = libc::isatty(STDOUT_FILENO) == 1;
        let stderr_is_tty: bool = libc::isatty(STDERR_FILENO) == 1;
        ret |= stdin_is_tty as u8;
        ret |= (stdout_is_tty as u8) << 1;
        ret |= (stderr_is_tty as u8) << 2;
    }
    ret
}

#[inline]
pub fn not_tty() -> bool {
    let res: u8 = get_tty_flag();
    res == 0
}

#[inline]
pub fn not_tty_fd(fd: libc::c_int) -> bool {
    unsafe { libc::isatty(fd) != 1 }
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
