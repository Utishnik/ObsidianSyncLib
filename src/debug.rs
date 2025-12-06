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

                if !(*left_val == *right_val)
                {
                    eprintln!();
                    eprintln!("{}══════════════════════════════════════════════════════════{}", RED, RESET);
                    eprintln!("{} ✗ ТЕСТ НЕ ПРОЙДЕН{}", RED, RESET);
                    eprintln!("{} Ожидалось: {:?}{}", YELLOW, right_val, RESET);
                    eprintln!("{} Получено:  {:?}{}", RED, left_val, RESET);
                    eprintln!("{}══════════════════════════════════════════════════════════{}\n", RED, RESET);
                    std::process::exit(1);
                }
                else
                {
                    eprintln!();
                    eprintln!("{}══════════════════════════════════════════════════════════{}", GREEN, RESET);
                    eprintln!("{} ✔ ТЕСТ ПРОЙДЕН{}", GREEN, RESET);
                    eprintln!("{} Ожидалось: {:?}{}", YELLOW, right_val, RESET);
                    eprintln!("{} Получено:  {:?}{}", GREEN, left_val, RESET);
                    eprintln!("{}══════════════════════════════════════════════════════════{}\n", GREEN, RESET);
                }
            }
        }
    };
}
