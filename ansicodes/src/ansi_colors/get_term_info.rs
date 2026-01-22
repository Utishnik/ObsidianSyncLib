//https://github.com/Stebalien/term/blob/master/src/terminfo/mod.rs

use crate::term_bd_info::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

pub type Result<T> = std::result::Result<T, ErrorUsed>;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
/// An error from parsing a terminfo entry
pub enum Error {
    /// The "magic" number at the start of the file was wrong.
    ///
    /// It should be `0x11A` (16bit numbers) or `0x21e` (32bit numbers)
    BadMagic(u16),
    /// The names in the file were not valid UTF-8.
    ///
    /// In theory these should only be ASCII, but to work with the Rust `str` type, we treat them
    /// as UTF-8. This is valid, except when a terminfo file decides to be invalid. This hasn't
    /// been encountered in the wild.
    NotUtf8(std::str::Utf8Error),
    /// The names section of the file was empty
    ShortNames,
    /// More boolean parameters are present in the file than this crate knows how to interpret.
    TooManyBools,
    /// More number parameters are present in the file than this crate knows how to interpret.
    TooManyNumbers,
    /// More string parameters are present in the file than this crate knows how to interpret.
    TooManyStrings,
    /// The length of some field was not >= -1.
    InvalidLength,
    /// The names table was missing a trailing null terminator.
    NamesMissingNull,
    /// The strings table was missing a trailing null terminator.
    StringsMissingNull,
}

/// An error arising from interacting with the terminal.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorUsed {
    /// Indicates an error from any underlying IO
    Io(io::Error),
    /// Indicates an error during terminfo parsing
    TerminfoParsing(String),
    /// Indicates an error expanding a parameterized string from the terminfo database
    ParameterizedExpansion(String),
    /// Indicates that the terminal does not support the requested operation.
    NotSupported,
    /// Indicates that the `TERM` environment variable was unset, and thus we were unable to detect
    /// which terminal we should be using.
    TermUnset,
    /// Indicates that we were unable to find a terminfo entry for the requested terminal.
    TerminfoEntryNotFound,
    /// Indicates that the cursor could not be moved to the requested position.
    CursorDestinationInvalid,
    /// Indicates that the terminal does not support displaying the requested color.
    ///
    /// This is like `NotSupported`, but more specific.
    ColorOutOfRange,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadMagic(v) => write!(f, "bad magic number {v:x} in terminfo header"),
            Self::ShortNames => f.write_str("no names exposed, need at least one"),
            Self::TooManyBools => f.write_str("more boolean properties than libterm knows about"),
            Self::TooManyNumbers => f.write_str("more number properties than libterm knows about"),
            Self::TooManyStrings => f.write_str("more string properties than libterm knows about"),
            Self::InvalidLength => f.write_str("invalid length field value, must be >= -1"),
            Self::NotUtf8(e) => e.fmt(f),
            Self::NamesMissingNull => f.write_str("names table missing NUL terminator"),
            Self::StringsMissingNull => f.write_str("string table missing NUL terminator"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TermInfo {
    /// Names for the terminal
    pub names: Vec<String>,
    /// Map of capability name to boolean value
    pub bools: HashMap<&'static str, bool>,
    /// Map of capability name to numeric value
    pub numbers: HashMap<&'static str, u32>,
    /// Map of capability name to raw (unexpanded) string
    pub strings: HashMap<&'static str, Vec<u8>>,
}

/// Returns true if the named terminal supports basic ANSI escape codes.
pub fn is_ansi(name: &str) -> bool {
    // SORTED! We binary search this.
    static ANSI_TERM_PREFIX: &[&str] = &[
        "Eterm", "ansi", "eterm", "iterm", "konsole", "linux", "mrxvt", "msyscon", "rxvt",
        "screen", "tmux", "xterm",
    ];
    match ANSI_TERM_PREFIX.binary_search(&name) {
        Ok(_) => true,
        Err(0) => false,
        Err(idx) => name.starts_with(ANSI_TERM_PREFIX[idx - 1]),
    }
}

#[cfg(windows)]
pub fn from_env() -> Result<TermInfo> {
    let term_var: Option<String> = env::var("TERM").ok();
    let term_name: Option<&str> = term_var.as_deref().or_else(|| {
        env::var("MSYSCON").ok().and_then(|s| {
            if s == "mintty.exe" {
                Some("msyscon")
            } else {
                None
            }
        })
    });

    #[cfg(windows)]
    {
        if term_name.is_none() && win::supports_ansi() {
            // Microsoft people seem to be fine with pretending to be xterm:
            // https://github.com/Microsoft/WSL/issues/1446
            // The basic ANSI fallback terminal will be uses.
            return TermInfo::from_name("xterm");
        }
    }

    if let Some(term_name) = term_name {
        TermInfo::from_name(term_name)
    } else {
        Err(ErrorUsed::TermUnset)
    }
}

#[cfg(windows)]
impl TermInfo {
    pub fn from_name(name: &str) -> Result<TermInfo> {
        if let Some(path) = get_dbpath_for_term(name) {
            match TermInfo::from_path(path) {
                Ok(term) => return Ok(term),
                // Skip IO Errors (e.g., permission denied).
                Err(ErrorUsed::Io(_)) => {}
                // Don't ignore malformed terminfo databases.
                Err(e) => return Err(e),
            }
        }
        // Basic ANSI fallback terminal.
        if is_ansi(name) {
            let mut strings: HashMap<&str, Vec<u8>> = HashMap::new();
            strings.insert("sgr0", b"\x1B[0m".to_vec());
            strings.insert("bold", b"\x1B[1m".to_vec());
            strings.insert("setaf", b"\x1B[3%p1%dm".to_vec());
            strings.insert("setab", b"\x1B[4%p1%dm".to_vec());

            let mut numbers: HashMap<&str, u32> = HashMap::new();
            numbers.insert("colors", 8);

            Ok(TermInfo {
                names: vec![name.to_owned()],
                bools: HashMap::new(),
                numbers,
                strings,
            })
        } else {
            Err(ErrorUsed::TerminfoEntryNotFound)
        }
    }
}

