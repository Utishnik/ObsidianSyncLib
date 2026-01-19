//https://github.com/Stebalien/term/blob/master/src/terminfo/mod.rs

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
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


