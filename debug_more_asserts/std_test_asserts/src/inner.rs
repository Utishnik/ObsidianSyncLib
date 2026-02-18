use core::fmt;
use more_debug_asserts::inner::*;
use obsidian_sync_lib::debug_eprintln;
//TODO нужно будет все ассерты перекопировать но с debug.rs подобным выводом и форматированием
#[cold]
#[track_caller]
#[inline(never)]
pub fn not_panic_assert_failed_impl(
    left: &dyn fmt::Debug,
    right: &dyn fmt::Debug,
    ty: AssertType,
    msg: Option<fmt::Arguments<'_>>,
) {
    let compare: &str = match ty {
        AssertType::Lt => "<",
        AssertType::Gt => ">",
        AssertType::Le => "<=",
        AssertType::Ge => ">=",
    };
    if let Some(msg) = msg {
        debug_eprintln!(
            "assertion failed: `(left {} right)`\n  left: `{:?}`,\n right: `{:?}`: {}",
            compare,
            left,
            right,
            msg,
        );
    } else {
        debug_eprintln!(
            "assertion failed: `(left {} right)`\n  left: `{:?}`,\n right: `{:?}`: {}",
            compare,
            left,
            right,
            "",
        );
    }
}

#[macro_export]
macro_rules! more_test_assert_templ {
    ($left:expr,$right:expr,&type_tst:expr) => {
        match (&$left, &$right)
        {
            (left_val, right_val) =>
            {
                const RED: &str = "\x1b[31m";
                const GREEN: &str = "\x1b[32m";
                const YELLOW: &str = "\x1b[33m";
                const RESET: &str = "\x1b[0m";
                let mut result_test: bool = false;
                use $crate::debug::debug_and_test_utils::Test;
                use $crate::debug::debug_and_test_utils::get_last_test;
                use $crate::debug::debug_and_test_utils::get_count_type_test;
                use $crate::debug::debug_and_test_utils::add_test;//fix
                use std::sync::atomic::Ordering;
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

                if *left_val == *right_val
                {
                    println!();
                    println!("{}══════════════════════════════════════════════════════════{}", RED, RESET);
                    println!("{} ✗ ТЕСТ НЕ ПРОЙДЕН{}", RED, RESET);
                    not_panic_assert_failed_impl(left_val, right_val,...);//todo
                    //println!("{} Результат не должен был равен: {:?}{}", YELLOW, right_val, RESET);//todo not_panic_assert_failed_impl
                    println!("{} Получено:  {:?}{}", RED, left_val, RESET);
                    println!("{}══════════════════════════════════════════════════════════{}\n", RED, RESET);
                    result_test=false;
                    //todo чекнуть почему при eprintln! может ломматься вывод
                }
                else
                {
                    println!();
                    println!("{}══════════════════════════════════════════════════════════{}", GREEN, RESET);
                    println!("{} ✔ ТЕСТ ПРОЙДЕН{}", GREEN, RESET);
                    println!("{} Результат не должен был равен: {:?}{}", YELLOW, right_val, RESET);
                    println!("{} Получено:  {:?}{}", GREEN, left_val, RESET);
                    println!("{}══════════════════════════════════════════════════════════{}\n", GREEN, RESET);
                    result_test=true;
                }
                test.result=result_test;
                test.typetest=get_count_type_test();//fix
                add_test(test);
            }
        }
    };
}
