use std::sync::Mutex;
use std::sync::Arc;
use std::sync::OnceLock;

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
pub struct Test
{
    pub result: bool,
    pub number: usize,
}

//maybe многопоточное тестирования хз нужно?
pub static TESTS: OnceLock<Arc<Mutex<Vec<Test>>>> = OnceLock::new();

fn get_tests() -> &'static Arc<Mutex<Vec<Test>>> 
{
    TESTS.get_or_init(|| Arc::new(Mutex::new(Vec::new())))
}

pub fn get_count_tests() -> usize
{
    let tests: &Arc<Mutex<Vec<Test>>> = get_tests();
    let guard: std::sync::MutexGuard<'_, Vec<Test>> = tests.lock().unwrap();
    let cnt: usize = guard.len();
    cnt
}

pub fn add_test(test: Test)
{
    let tests: &Arc<Mutex<Vec<Test>>> = get_tests();
    let mut guard: std::sync::MutexGuard<'_, Vec<Test>> = tests.lock().unwrap();
    guard.push(test);
}

pub fn get_test(index: usize) -> Option<Test>
{
    let tests: &Arc<Mutex<Vec<Test>>> = get_tests();
    let guard: std::sync::MutexGuard<'_, Vec<Test>> = tests.lock().unwrap();
    let test: Option<Test> = guard.get(index).cloned();
    test
}

//todo get_test_slice

pub fn get_last_test() -> Option<Test>
{
    let tests: &Arc<Mutex<Vec<Test>>> = get_tests();
    let guard: std::sync::MutexGuard<'_, Vec<Test>> = tests.lock().unwrap();
    guard.last().cloned()
}

pub fn result_list()
{
    let cnt_test: usize=get_count_tests();
    let mut i: usize=0;
    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";
    //c like
    while i < cnt_test
    {
        let g_test: Option<Test> = get_test(i);
        match g_test
        {
            None => {},
            Some(x) =>
            {
                if x.result
                {
                    println!("{}",GREEN);
                }
                else 
                {
                    println!("{}",RED);
                }
                println!("index: {} --- status: {}",x.number,x.result);
                print!("{}",RESET);
            }
        }
        i+=1;
    }
}

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
                let last_test_option: Option<Test> = get_last_test();
                let mut test: Test;
                match last_test_option
                {
                    None => 
                    {
                        test = Test {result: false, number: 1};
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
                add_test(test);
            }
        }
    };
}
