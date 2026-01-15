use backtrace::Backtrace;
use core::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::OnceLock;

static hand_log_panic_hook: Arc<
    OnceLock<Box<dyn Fn(core::fmt::Formatter<'static>) + Sync + Send>>,
> = Arc::new(OnceLock::new());

pub fn set_hand_log_panic_hook() {}

pub fn attach_panic_hook() {
    let prev: Box<dyn Fn(&std::panic::PanicHookInfo<'_>) + Send + Sync> = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let mut err: String = String::default();
        if let Some(x) = info.payload().downcast_ref::<String>() {
            err = x.clone();
        } else if let Some(x) = info.payload().downcast_ref::<&str>() {
            err = x.to_string();
        }
        let thread: std::thread::Thread = std::thread::current();
        let thread: &str = thread.name().unwrap_or("<unnamed>");
        if let Some(location) = info.location() {
            log!( format!(
                "{}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            );"error"; msg= "PANIC", err, thread)
        } else {
            error!("PANIC", err, thread)
        }

        prev(info)
    }));
}

pub fn is_crate(_symbol: String) {}

pub fn get_caller_info() {
    let _bt: Backtrace = Backtrace::new();
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
