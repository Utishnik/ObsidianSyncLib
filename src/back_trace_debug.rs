use std::path::Path;

use crate::{debug::*, debug_println};
use backtrace::Backtrace;
use backtrace::SymbolName;
use std::path::PathBuf;

pub fn is_crate(symbol: String) {}

pub fn get_caller_info() {
    let bt: Backtrace = Backtrace::new();
    let mut vec_sym: Vec<String> = Vec::new();
    let mut vec_file: Vec<PathBuf> = Vec::new();
    let mut sym: Option<String> = None;
    let mut file: PathBuf = PathBuf::new();

    let mut skip_count: usize = 0;

    backtrace::trace(|frame: &backtrace::Frame| {
        if skip_count > 0 {
            skip_count -= 1;
            return true;
        }

        backtrace::resolve_frame(frame, |symbol| {
            if let Some(name) = symbol.name() {
                sym = Some(name.to_string());
                vec_sym.push(sym.clone().unwrap());
                println!("{}", name.to_string());
            }
            if let Some(filename) = symbol.filename() {
                file = filename.to_path_buf();
                vec_file.push(file.clone());
                println!("{}", filename.display());
            }
        });
        true
    });

    println!(
        "{}\t {}",
        sym.unwrap(),
        file.as_os_str().to_str().unwrap().to_string()
    );
}
