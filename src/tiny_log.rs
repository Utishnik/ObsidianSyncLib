//TODO: ЗАМЕНИТЬ АНСИ ЧЕРЕЗ WRITE НА ЛИБУ

pub use serde_json;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs::File, io::IsTerminal};

use std::sync::{Condvar, Mutex};
use std::thread;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SyncState {
    Flushing,
    Writing,
    Waiting,
}

#[allow(unused)]
pub struct TCQueue {
    pub buffer: Vec<u8>,
    pub tmp: Vec<u8>,
    pub sync_state: SyncState,
    pub exiting: bool,
    pub handle: Option<thread::JoinHandle<()>>,
}

pub static FLUSHING: Condvar = Condvar::new();

pub static LOG_WRITER: Mutex<TCQueue> = Mutex::new(TCQueue {
    buffer: Vec::new(),
    tmp: Vec::new(),
    sync_state: SyncState::Waiting,
    exiting: false,
    handle: None,
});

const BUFFER_CAPACITY: usize = 4096;
pub fn pretty_print_buffer(buffer: &[u8]) {
    let mut stdout: std::io::Stdout = std::io::stdout();
    let mut in_quote: bool = false;
    let mut prev_escape: bool = false;
    for line in buffer.split(|x| {
        if *x == b'"' && !prev_escape {
            in_quote ^= true;
        }
        prev_escape = *x == b'\\';

        if *x == b'\n' && !in_quote {
            in_quote = false;
            prev_escape = false;
            true
        } else {
            false
        }
    }) {
        if line.is_empty() {
            continue;
        }
        //         eprintln!("[[\n{}\n]]\n\n",
        //             unsafe {std::str::from_utf8_unchecked(line)}
        // );
        let mut out: std::io::StdoutLock<'_> = stdout.lock();
        let mut in_quote: bool = false;
        let mut last_escape: bool = false;
        for (key, value) in line
            .split(|&ch| {
                let escaped = last_escape;
                last_escape = false;
                if !in_quote && ch == b' ' {
                    return true;
                }
                if in_quote && ch == b'\\' {
                    last_escape = !escaped;
                    return false;
                }
                if ch == b'"' {
                    in_quote ^= !escaped;
                    return false;
                }
                false
            })
            .filter_map(|pair| unsafe { std::str::from_utf8_unchecked(pair) }.split_once('='))
        {
            if key == "ts" {
                let time: &str = if let Some((_, time)) = value.split_once('T') {
                    time.trim_end_matches('Z')
                } else {
                    value.trim_end_matches('Z')
                };
                write!(&mut out, "\x1b[38;5;246m {}\x1b[0m", time).ok();
                continue;
            }
            if key == "caller" {
                write!(&mut out, "\x1b[38;5;144m@{}\x1b[0m", value).ok();
                continue;
            }
            if key == "level" {
                if value == "info" {
                    out.write_all(b"\x1b[38;5;246mINFO\x1b[0m").ok();
                } else if value == "warn" {
                    out.write_all(b"\x1b[38;5;227mWARN\x1b[0m").ok();
                } else if value == "error" {
                    out.write_all(b"\x1b[38;5;197m ERR\x1b[0m").ok();
                } else {
                    write!(&mut out, "{:>5}", value).ok();
                }
                continue;
            }
            if key == "msg" {
                if value.starts_with('"') {
                    if let Ok(value) = serde_json::from_str::<String>(&value) {
                        write!(&mut out, " {}", value).ok();
                    } else {
                        write!(&mut out, " {}", value).ok();
                    }
                } else {
                    write!(&mut out, " {}", value).ok();
                }
                continue;
            }
            if key == "err" {
                write!(&mut out, "\x1b[38;5;197m err").ok();
            } else {
                write!(&mut out, "\x1b[38;5;110m {}", key).ok();
            }
            write!(&mut out, "\x1b[38;5;249m=").ok();

            // if value.starts_with('"') && value.contains(' ') {
            //     let val =
            //         serde_json::from_str::<String>(&value).unwrap_or_else(|_| value.to_string());
            //     let x = val.as_bytes();
            //     write!(&mut out, "`").ok();
            //     let mut s = 0;
            //     let inquote = false;
            //     let mut indent = 0u32;
            //     for (i, &ch) in x.iter().enumerate() {
            //         match ch {
            //             b'{' => {
            //                 if !inquote {
            //                     indent += 2;
            //                     out.write_all(&x[s..i + 1]).ok();
            //                     s = i + 1;
            //                     write!(&mut out, "\n").ok();
            //                     for _ in 0..indent {
            //                         out.write(b" ").ok();
            //                     }
            //                 }
            //             }
            //             b'}' => {
            //                 if !inquote {
            //                     indent = indent.saturating_sub(2);
            //                     out.write_all(&x[s..i + 1]).ok();
            //                     s = i + 1;
            //                     if indent > 0 {
            //                         write!(&mut out, "\n").ok();
            //                         for _ in 0..indent {
            //                             out.write(b" ").ok();
            //                         }
            //                     }
            //                 }
            //             }
            //             b'"' => {

            //             }
            //             b',' => {
            //                 out.write_all(&x[s..i + 1]).ok();
            //                 s = i + 1;
            //                 write!(&mut out, "\n").ok();
            //                 for _ in 0..indent {
            //                     out.write(b" ").ok();
            //                 }
            //             }
            //             _ => {}
            //         }
            //     }
            //     out.write_all(&x[s..]).ok();
            //     write!(&mut out, "`").ok();
            // } else
            if value.starts_with('"') {
                if let Ok(value) = serde_json::from_str::<String>(&value) {
                    write!(&mut out, "{}", value).ok();
                } else {
                    write!(&mut out, "{}", value).ok();
                }
            } else {
                write!(&mut out, "{}", value).ok();
            }
            write!(&mut out, "\x1b[0m").ok();
        }
        out.write(&[b'\n']).ok();
    }
    stdout.flush().ok();
}

fn stdout_sync_thread() {
    let mut max_size: usize = 0;
    let mut buffer: Vec<u8> = Vec::<u8>::with_capacity(BUFFER_CAPACITY);
    let mut wait_flushers: bool = false;
    loop {
        let mut queue: std::sync::MutexGuard<'_, TCQueue> = LOG_WRITER.lock().unwrap();
        if queue.sync_state == SyncState::Flushing {
            wait_flushers = true;
        }
        if queue.buffer.is_empty() {
            queue.sync_state = SyncState::Waiting;
            let exiting: bool = queue.exiting;
            if wait_flushers {
                wait_flushers = false;
                FLUSHING.notify_all();
            }
            drop(queue);
            if exiting {
                return;
            }
            std::thread::park();
            continue;
        }
        buffer.clear();
        std::mem::swap(&mut buffer, &mut queue.buffer);
        if buffer.len() > max_size {
            max_size = buffer.len();
        }
        queue.sync_state = SyncState::Writing;
        drop(queue);
        let mut stdout: std::io::Stdout = std::io::stdout();
        stdout.write_all(&buffer).ok();
    }
}

fn stdout_sync_thread_pretty() {
    let mut max_size: usize = 0;
    let mut buffer: Vec<u8> = Vec::<u8>::with_capacity(BUFFER_CAPACITY);
    let mut wait_flushers: bool = false;
    loop {
        let mut queue: std::sync::MutexGuard<'_, TCQueue> = LOG_WRITER.lock().unwrap();
        if queue.sync_state == SyncState::Flushing {
            wait_flushers = true;
        }
        if queue.buffer.is_empty() {
            queue.sync_state = SyncState::Waiting;
            let exiting: bool = queue.exiting;
            if wait_flushers {
                wait_flushers = false;
                FLUSHING.notify_all();
            }
            drop(queue);
            if exiting {
                return;
            }
            std::thread::park();
            continue;
        }
        buffer.clear();
        std::mem::swap(&mut buffer, &mut queue.buffer);
        if buffer.len() > max_size {
            max_size = buffer.len();
        }
        queue.sync_state = SyncState::Writing;
        drop(queue);
        pretty_print_buffer(&buffer);
    }
}
fn exit_logger() {
    let handle: Option<thread::JoinHandle<()>> = {
        let mut queue: std::sync::MutexGuard<'_, TCQueue> = LOG_WRITER.lock().unwrap();
        queue.exiting = true;
        queue.handle.take()
    };
    if let Some(handle) = handle {
        handle.thread().unpark();
        if let Ok(_) = handle.join() {}
    }
}
pub struct LoggerGuard {}

impl LoggerGuard {
    pub fn flush(&self) {
        let mut queue: std::sync::MutexGuard<'_, TCQueue> = LOG_WRITER.lock().unwrap();
        if let Some(handle) = &queue.handle {
            handle.thread().unpark();
        } else {
            return;
        }
        queue.sync_state = SyncState::Flushing;
        while queue.sync_state != SyncState::Waiting {
            queue = FLUSHING.wait(queue).unwrap();
        }
    }
}

impl Drop for LoggerGuard {
    fn drop(&mut self) {
        exit_logger();
    }
}
const LOG_FILE_TARGET_SIZE: usize = 1024 * 1024 * 16; // 16 MB

fn directory_sync_thread(path: Box<str>) {
    let mut pathbuf: PathBuf = PathBuf::from_str(&path).expect("XLOG invalid PATH");
    if let Err(err) = std::fs::create_dir_all(&pathbuf) {
        panic!("XLOG: {} {:?}", path, err);
    }
    let mut current: u32 = std::fs::read_dir(&pathbuf)
        .expect("PATH")
        .filter_map(|entry: Result<std::fs::DirEntry, std::io::Error>| {
            entry.ok()?.file_name().to_str()?.parse::<u32>().ok()
        })
        .max()
        .unwrap_or(0);
    pathbuf.push(&current.to_string());
    let (mut written, mut file) = if let Ok(meta) = std::fs::metadata(&pathbuf) {
        (
            meta.len() as usize,
            std::fs::OpenOptions::new()
                .append(true)
                .open(&pathbuf)
                .expect("XLOG SYNC THREAD: failed to open log"),
        )
    } else {
        (
            0,
            File::create(&pathbuf).expect("XLOG SYNC THREAD: failed to open log"),
        )
    };
    pathbuf.pop();
    let mut max_size: usize = 0;
    let mut buffer: Vec<u8> = Vec::<u8>::with_capacity(BUFFER_CAPACITY);
    loop {
        if written >= LOG_FILE_TARGET_SIZE {
            current += 1;
            pathbuf.push(&current.to_string());
            file = File::create(&pathbuf).expect("XLOG SYNC THREAD: failed to open log");
            pathbuf.pop();
            written = 0;
        }

        let mut queue: std::sync::MutexGuard<'_, TCQueue> = LOG_WRITER.lock().unwrap();
        if queue.buffer.is_empty() {
            queue.sync_state = SyncState::Waiting;
            let exiting = queue.exiting;
            drop(queue);
            if exiting {
                return;
            }
            std::thread::park();
            continue;
        }
        buffer.clear();
        std::mem::swap(&mut buffer, &mut queue.buffer);
        if buffer.len() > max_size {
            max_size = buffer.len();
        }
        queue.sync_state = SyncState::Writing;
        drop(queue);
        file.write_all(&mut buffer).unwrap();
        written += buffer.len();
    }
}
fn file_sync_thread(mut file: std::fs::File) {
    let mut max_size: usize = 0;
    let mut buffer: Vec<u8> = Vec::<u8>::with_capacity(BUFFER_CAPACITY);
    let mut wait_flushers: bool = false;
    loop {
        let mut queue: std::sync::MutexGuard<'_, TCQueue> = LOG_WRITER.lock().unwrap();
        if queue.sync_state == SyncState::Flushing {
            wait_flushers = true;
        }
        if queue.buffer.is_empty() {
            queue.sync_state = SyncState::Waiting;
            let exiting: bool = queue.exiting;
            if wait_flushers {
                wait_flushers = false;
                FLUSHING.notify_all();
            }
            drop(queue);
            if exiting {
                return;
            }
            std::thread::park();
            continue;
        }
        buffer.clear();
        std::mem::swap(&mut buffer, &mut queue.buffer);
        if buffer.len() > max_size {
            max_size = buffer.len();
        }
        queue.sync_state = SyncState::Writing;
        drop(queue);
        file.write_all(&buffer).ok();
    }
}

#[must_use]
pub fn init_stdout_logger() -> LoggerGuard {
    if std::io::stdout().is_terminal() {
        init_logger(stdout_sync_thread_pretty)
    } else {
        init_logger(stdout_sync_thread)
    }
}

#[must_use]
pub fn init_file_logger(file: &str) -> LoggerGuard {
    let path: String = file.to_string();
    init_logger(move || {
        file_sync_thread(
            std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(&path)
                .expect("FAILED to open log file"),
        )
    })
}

#[must_use]
pub fn init_directory_logger(dir: &str) -> LoggerGuard {
    let dir: Box<str> = dir.into();
    init_logger(move || directory_sync_thread(dir))
}

pub fn attach_panic_hook() {
    let prev: Box<dyn Fn(&std::panic::PanicHookInfo<'_>) + Send + Sync> = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let mut err: String = String::default();
        if let Some(x) = info.payload().downcast_ref::<String>() {
            err = x.clone();
        } else if let Some(x) = info.payload().downcast_ref::<&str>() {
            err = x.to_string();
        }
        let thread: thread::Thread = std::thread::current();
        let thread: &str = thread.name().unwrap_or("<unnamed>");
        if let Some(location) = info.location() {
            crate::log!( format!(
                "{}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            );"error"; msg= "PANIC", err, thread)
        } else {
            crate::error!("PANIC", err, thread)
        }

        prev(info)
    }));
}
fn init_logger(syncfn: impl FnOnce() + Send + 'static) -> LoggerGuard {
    LOG_WRITER.lock().unwrap().handle = Some(std::thread::spawn(syncfn));
    LoggerGuard {}
}

#[macro_export]
macro_rules! info {
    ($msg: literal $($args:tt)*) => {$crate::log!(
        concat!(file!(), ":",line!());"info"; msg= $msg $($args)*)
    };
    ($($args:tt)*) => {$crate::log!(
        concat!(file!(), ":",line!());"info"; $($args)*)};
}

#[macro_export]
macro_rules! warn {
    ($msg: literal $($args:tt)*) => {$crate::log!(
        concat!(file!(), ":",line!());"warn"; msg= $msg $($args)*)
    };
    ($($args:tt)*) => {$crate::log!(
        concat!(file!(), ":",line!());"warn"; $($args)*)}
}

#[macro_export]
macro_rules! error {
    ($msg: literal $($args:tt)*) => {$crate::log!(
        concat!(file!(), ":",line!());"error"; msg= $msg $($args)*)
    };
    ($($args:tt)*) => {$crate::log!(
        concat!(file!(), ":",line!()); "error"; $($args)*)}
}
pub use time::OffsetDateTime;

#[inline(never)]
pub fn __fmt_init(out: &mut Vec<u8>, t: &OffsetDateTime) {
    let mut buf: itoa::Buffer = itoa::Buffer::new();
    out.extend_from_slice(b" ts=");
    out.extend_from_slice(buf.format(t.year()).as_bytes());
    out.push(b'-');
    let mut a = |out: &mut Vec<u8>, t: u8| {
        let bytes: &[u8] = buf.format(t).as_bytes();
        if bytes.len() == 1 {
            out.push(b'0');
        }
        out.extend_from_slice(buf.format(t).as_bytes());
    };
    a(out, t.month() as u8);
    out.push(b'-');
    a(out, t.day());
    out.push(b'T');
    a(out, t.hour());
    out.push(b':');
    a(out, t.minute());
    out.push(b':');
    a(out, t.second());
    out.push(b'Z');
}

#[macro_export]
macro_rules! log {
    ($caller:expr; $level:literal; $($x: ident $( = $value:expr)?),* $(,)?) => {{
        let t = $crate::tiny_log::OffsetDateTime::now_utc();
        let mut log_guard = $crate::tiny_log::LOG_WRITER.lock().unwrap();
        let queue: &mut $crate::tiny_log::TCQueue =  &mut log_guard;
        use $crate::tiny_log::LoggerDisplayDispatch;
        queue.buffer.extend_from_slice(
            concat!("level=", $level, " caller=").as_bytes()
        );
        queue.buffer.extend_from_slice(
            $caller.as_bytes()
        );
        $crate::tiny_log::__fmt_init(&mut queue.buffer, &t);
        $(
            queue.buffer.extend_from_slice(concat!(" ", stringify!($x), "=").as_bytes());
            (&&&&&&$crate::tiny_log::DWrap(&$crate::__v!($x $(;$value)?))).render(queue);
        )*
        queue.buffer.push(b'\n');
        if cfg!(test) {
            $crate::tiny_log::pretty_print_buffer(&queue.buffer);
            queue.buffer.clear();
        } else {
            if queue.sync_state != $crate::tiny_log::SyncState::Writing {
                if let Some(handle) = &queue.handle {
                    handle.thread().unpark();
                }
            }
        }
    }}
}

#[doc(hidden)]
#[macro_export]
macro_rules! __v {
    ($x:ident) => {
        $x
    };
    ($x:ident; $value: expr) => {
        $value
    };
}

pub trait SafeDisplay: Display {}
impl SafeDisplay for i8 {}
impl SafeDisplay for i16 {}
impl SafeDisplay for i32 {}
impl SafeDisplay for i64 {}
impl SafeDisplay for u8 {}
impl SafeDisplay for u16 {}
impl SafeDisplay for u32 {}
impl SafeDisplay for u64 {}
impl SafeDisplay for f32 {}
impl SafeDisplay for f64 {}

pub struct DWrap<T>(pub T);
pub trait LoggerDisplayDispatch {
    fn render(&self, queue: &mut TCQueue);
}

impl<T: itoa::Integer + Copy> LoggerDisplayDispatch for &&&&DWrap<&T> {
    #[inline]
    fn render(&self, queue: &mut TCQueue) {
        let mut buffer: itoa::Buffer = itoa::Buffer::new();
        queue
            .buffer
            .extend_from_slice(buffer.format(*self.0).as_bytes());
        // itoa::write(&mut queue.buffer, *self.0).ok();
    }
}

impl<T: SafeDisplay> LoggerDisplayDispatch for &&&DWrap<&T> {
    #[inline]
    fn render(&self, queue: &mut TCQueue) {
        write!(&mut queue.buffer, "{}", self.0).ok();
    }
}

impl<T: AsRef<str>> LoggerDisplayDispatch for &&DWrap<T> {
    fn render(&self, queue: &mut TCQueue) {
        let text: &str = self.0.as_ref();
        if text.contains(|ch| ch < '#' || ch > '~') {
            serde_json::to_writer(&mut queue.buffer, &text).ok();
        } else {
            queue.buffer.extend_from_slice(text.as_bytes());
        }
    }
}

use std::fmt::{Debug, Display};
impl<T: Display> LoggerDisplayDispatch for &DWrap<T> {
    fn render(&self, queue: &mut TCQueue) {
        queue.tmp.clear();
        write!(&mut queue.tmp, "{}", self.0).ok();
        if queue.tmp.iter().any(|&ch| ch < b'#' || ch > b'~') {
            serde_json::to_writer(&mut queue.buffer, unsafe {
                std::str::from_utf8_unchecked(&queue.tmp)
            })
            .ok();
        } else {
            queue.buffer.extend_from_slice(&queue.tmp);
        }
    }
}

impl<T: Debug> LoggerDisplayDispatch for DWrap<T> {
    fn render(&self, queue: &mut TCQueue) {
        queue.tmp.clear();
        write!(&mut queue.tmp, "{:?}", self.0).ok();
        if queue.tmp.iter().any(|&ch| ch < b'#' || ch > b'~') {
            serde_json::to_writer(&mut queue.buffer, unsafe {
                std::str::from_utf8_unchecked(&queue.tmp)
            })
            .ok();
        } else {
            queue.buffer.extend_from_slice(&queue.tmp);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stdout_logger() {
        warn!(x = 34);
    }
    // #[test]
    // fn file_logger() {
    //     warn!("hello");
    //     struct BillyBob {
    //         x: u32
    //     }
    //     let b = BillyBob {
    //         x: 3
    //     };
    //     warn!(b.x, "asdf");

    // }
}
