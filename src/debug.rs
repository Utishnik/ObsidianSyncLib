use crate::debug_println;
use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::i64::MAX;
use std::io::Write;
use std::num::ParseIntError;
use std::ops::Deref;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;

pub struct ArgsFmt {
    args: Vec<String>,
    cnt_args: usize,
}

pub enum StreamPrint {
    StdOut,
    StdErr,
}

impl StreamPrint {
    pub fn as_str(&self) -> String {
        match self {
            StreamPrint::StdErr => "stderr".to_string(),
            StreamPrint::StdOut => "stdout".to_string(),
        }
    }
}

impl StreamPrint {
    pub fn str_to_self(&self, str: &str) -> Result<StreamPrint, ()> {
        let res: StreamPrint = match str {
            "stderr" => StreamPrint::StdErr,
            "stdout" => StreamPrint::StdOut,
            _ => {
                return Err(());
            }
        };
        Ok(res)
    }
}

impl ArgsFmt {
    pub fn new() -> Self {
        Self {
            args: Vec::new(),
            cnt_args: 0,
        }
    }

    pub fn add<T: fmt::Display>(&mut self, arg: T) {
        self.args.push(format!("{}", arg));
        self.cnt_args += 1;
    }

    pub fn get_len(&self) -> usize {
        self.cnt_args
    }

    pub fn debug_print(&self) {
        debug_println!("ArgsFmt args: ");
        for i in self.args.iter() {
            debug_println!("{};", i);
        }
    }

    pub fn first(&self) -> Option<String> {
        self.args.first().cloned()
    } //todo использовать не только first но и другие значение например следуйщиее целое число
      //это сколько выводов показать или каких их цветом
      //patterns(несколько их может быть не только один вывод) после них количество выводов каждого патерна
      //I64SIZEMAX значит все i64 используется так как + значит первый n выводов - последние n выводов
      //и потом идут цвета каждого вывода
      //first будет означать кого типа выводить обычных stdout или поток ошибок

    pub fn get_args_owned(&self) -> Vec<String> {
        self.args.clone()
    }

    pub fn get_args_ref(&self) -> &[String] {
        &self.args
    }
}

impl Iterator for ArgsFmt {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let len: usize = self.get_len();
        let mut i: usize = len;
        'out: while i > 0 {
            let ret: Option<&String> = self.get_args_ref().get(i).clone();
            if ret.is_none() {
                break 'out;
            } else {
                i += 1;
                return Some(ret.map_or("zxc", |v| v).to_string());
            }
        }
        None
    }
}

impl Default for ArgsFmt {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! argsfmt
{
    ($($arg:expr),* $(,)?) =>
    {{
        let mut args = ArgsFmt::new();
        $(args.add($arg);)*
        args
    }};
}

fn iterate_cnt_parse(args: &[String]) -> Option<Vec<i64>> {
    let mut result: Vec<i64> = Vec::new();
    for i in args.iter() {
        let item: Result<i64, ParseIntError> = i.parse();
        if item.is_err() {
            break;
        }
        let val: i64 = item.unwrap(); //safe
        result.push(val);
    }
    Some(result)
}

fn iterate_pattern_parse(args: &[String]) -> Option<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    for i in args.iter() {
        let item: Result<i64, ParseIntError> = i.parse();
        if item.is_ok() {
            break;
        }
        let val: &String = i;
        result.push(val.to_string());
    }
    Some(result)
}

//todo для всех debug print/eprint метод сделать ввод в фаил и возможность воспроизведения

//todo функции серелизации и десерелизации stdout stderr

pub fn fmt_args_parse(args: &[String]) -> Result<(), String> {
    let mut stream_type: StreamPrint = StreamPrint::StdOut;
    for item in args.to_vec().iter().enumerate() {
        let (i, str): (usize, &String) = item;
        if i == 0 {
            let state: Result<StreamPrint, ()> = stream_type.str_to_self(str);
            match state {
                Ok(v) => stream_type = v,
                Err(_) => {
                    return Err("parse stream error".to_string());
                }
            }
        } else {
        }
    }
    Ok(())
}

fn private_printing_manage(args: fmt::Arguments) {
    //let patern: String = args.as_str().unwrap().to_string();//contains
    let argsfmt: ArgsFmt = argsfmt!(args);
    let args: Vec<String> = argsfmt.get_args_owned();
    let patern: String = " ".to_string();
    let str_args: String = patern.to_string();
    let find_state: bool = false;
}

#[macro_export]
macro_rules! printing_manage {
    ($($arg:tt)*) => {
        private_printing_manage(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) =>
    {
        #[cfg(debug_assertions)]
        {
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! debug_eprintln {
    ($($arg:tt)*) =>
    {
        #[cfg(debug_assertions)]
        {
            eprintln!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! debug_println_fileinfo {
    ($($arg:tt)*) =>
    {
        #[cfg(debug_assertions)]
        {
            println!("[ {} : {} ]",file!(),line!());
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! debug_eprintln_fileinfo {
    ($($arg:tt)*) =>
    {
        #[cfg(debug_assertions)]
        {
            eprintln!("[ {} : {} ]",file!(),line!());
            eprintln!($($arg)*);
        }
    };
}
//думаю клонирование дешевле либо +- также чем удерживать блокировку
#[derive(Clone)]
pub struct Test {
    pub result: bool,
    pub number: usize,
    pub typetest: usize,
    //pub cnt_types: usize,
}

//maybe многопоточное тестирования хз нужно?
pub static TESTS: OnceLock<Arc<Mutex<Vec<Test>>>> = OnceLock::new();
pub static type_test: AtomicUsize = AtomicUsize::new(0);

fn get_tests() -> &'static Arc<Mutex<Vec<Test>>> {
    TESTS.get_or_init(|| Arc::new(Mutex::new(Vec::new())))
}

pub fn get_count_tests() -> usize {
    let tests: &Arc<Mutex<Vec<Test>>> = get_tests();
    let guard: std::sync::MutexGuard<'_, Vec<Test>> = tests.lock().unwrap();
    let cnt: usize = guard.len();
    cnt
}

pub fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn successes_beh() {
    let mut i: usize = 0;
    let cnt_test: usize = get_count_tests();
    let mut suc_test: usize = 0;
    let mut not_suc_test: usize = 0;
    //c like
    while i < cnt_test {
        let g_test: Option<Test> = get_test(i);
        match g_test {
            None => {}
            Some(x) => {
                if x.result {
                    suc_test += 1;
                } else {
                    not_suc_test += 1;
                }
            }
        }
        i += 1;
    }
    let mut diff: usize = 0;
    if suc_test > not_suc_test {
        diff = suc_test - not_suc_test;
    } else {
        diff = not_suc_test - suc_test;
    }
    if not_suc_test > 0 {
        std::process::exit(diff.try_into().unwrap_or(1));
    }
}

pub fn add_test(test: Test) {
    let tests: &Arc<Mutex<Vec<Test>>> = get_tests();
    let mut guard: std::sync::MutexGuard<'_, Vec<Test>> = tests.lock().unwrap();
    guard.push(test);
}

pub fn add_type_test() {
    type_test.fetch_add(1, Ordering::Relaxed);
}

pub fn get_type_test() -> usize {
    type_test.load(Ordering::Relaxed)
}

pub fn get_test(index: usize) -> Option<Test> {
    let tests: &Arc<Mutex<Vec<Test>>> = get_tests();
    let guard: std::sync::MutexGuard<'_, Vec<Test>> = tests.lock().unwrap();
    let test: Option<Test> = guard.get(index).cloned();
    test
}

//todo get_test_slice

pub fn get_last_test() -> Option<Test> {
    let tests: &Arc<Mutex<Vec<Test>>> = get_tests();
    let guard: std::sync::MutexGuard<'_, Vec<Test>> = tests.lock().unwrap();
    guard.last().cloned()
}

pub fn result_list() -> bool {
    let cnt_test: usize = get_count_tests();
    let mut i: usize = 0;
    const RED: &str = "\x1b[31m"; //todo по идеи можно заменить на константы глобальные в самом фаиле
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";
    let mut successfully: bool = true;
    //c like
    while i < cnt_test {
        let g_test: Option<Test> = get_test(i);
        match g_test {
            None => {}
            Some(x) => {
                if x.result {
                    println!("{}", GREEN);
                } else {
                    println!("{}", RED);
                    successfully = false;
                }
                println!("index: {} --- status: {}", x.number, x.result);
                print!("{}", RESET);
            }
        }
        i += 1;
    }
    successfully
}

pub fn dump_result_list(path: String) -> Result<(), std::io::Error> //todo: сделать асинхронной? типо тестов может быть очень много
//и результаты всех нужно дампать
//с другой стороны тут большая логическая нагрзука а не просто запросить данные
{
    let mut file: File = File::create(path)?;
    let cnt_test: usize = get_count_tests();
    const RED: &str = "\x1b[31m"; //todo по идеи можно заменить на константы глобальные в самом фаиле
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";
    let mut i: usize = 0;

    while i < cnt_test {
        let g_test: Option<Test> = get_test(i);
        match g_test {
            None => {}
            Some(x) => {
                if x.result {
                    file.write_fmt(format_args!("{}", GREEN))?;
                } else {
                    file.write_fmt(format_args!("{}", RED))?;
                }
                file.write_fmt(format_args!("index: {} --- status: {}", x.number, x.result))?;
                file.write_fmt(format_args!("{}", RESET))?;
            }
        }
        i += 1;
    }
    Ok(()) //TODO доделать там нумерацию фаилов пред проверки всякие и тд и наверное чтоб он автоматически взависомти от
           //теста в нужную директорию
}

//todo: pub fn dump_list_print

//ебаный раст засирает assert_eq поэтому пишем свой
#[macro_export]
macro_rules! test_assert {
    ($left:expr,$right:expr) => {
        match (&$left, &$right)
        {
            (left_val, right_val) =>
            {
                const RED: &str = "\x1b[31m";
                const GREEN: &str = "\x1b[32m";
                const YELLOW: &str = "\x1b[33m";
                const RESET: &str = "\x1b[0m";
                let mut result_test: bool = false;
                use crate::debug::*;
                use std::sync::atomic::Ordering;
                //$crate::debug::* может так лучше?
                let last_test_option: Option<Test> = get_last_test();
                let mut test: Test;
                match last_test_option
                {
                    None =>
                    {
                        test = Test {result: false, number: 1,typetest: 0};
                    },
                    Some(x) =>
                    {
                        test=x.clone();
                        test.number=x.number+1;
                    }
                }

                if !(*left_val == *right_val)
                {
                    eprintln!();
                    eprintln!("{}══════════════════════════════════════════════════════════{}", RED, RESET);
                    eprintln!("{} ✗ ТЕСТ НЕ ПРОЙДЕН{}", RED, RESET);
                    eprintln!("{} Ожидалось: {:?}{}", YELLOW, right_val, RESET);
                    eprintln!("{} Получено:  {:?}{}", RED, left_val, RESET);
                    eprintln!("{}══════════════════════════════════════════════════════════{}\n", RED, RESET);
                    result_test=false;
                }
                else
                {
                    eprintln!();
                    eprintln!("{}══════════════════════════════════════════════════════════{}", GREEN, RESET);
                    eprintln!("{} ✔ ТЕСТ ПРОЙДЕН{}", GREEN, RESET);
                    eprintln!("{} Ожидалось: {:?}{}", YELLOW, right_val, RESET);
                    eprintln!("{} Получено:  {:?}{}", GREEN, left_val, RESET);
                    eprintln!("{}══════════════════════════════════════════════════════════{}\n", GREEN, RESET);
                    result_test=true;
                }
                test.result=result_test;
                test.typetest=get_type_test();
                add_test(test);
            }
        }
    };
}
