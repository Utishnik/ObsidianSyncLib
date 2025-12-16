use std::collections::HashSet;
use std::default;
use std::time::Instant;

use crate::debug_eprintln_fileinfo;
use crate::debug_println;
use crate::debug_println_fileinfo;

fn days_to_ms(days: u128) -> u128 {
    days * 24 * 3600 * 1000
}
fn hours_to_ms(hours: u128) -> u128 {
    hours * 3600 * 1000
}
fn minutes_to_ms(minutes: u128) -> u128 {
    minutes * 60 * 1000
}
fn seconds_to_ms(seconds: u128) -> u128 {
    seconds * 1000
}
fn ms_to_ms(miliseconds: u128) -> u128{
    miliseconds * 1
}

pub fn check_period_passed(prev_time: Instant, diff_time: u128) -> bool {
    let duration: std::time::Duration = prev_time.elapsed();
    if duration.as_millis() > diff_time {
        //нужно что то точнее милисекунд?
        debug_println_fileinfo!("check_time_diff time diff: {}", duration.as_millis());
        return true;
    }
    false
}
//память нынче дорогая(
fn time_period_set(d: u128,h: u128, m: u128, s: u128, ms: u128) -> u128 {
    let result: u128 = days_to_ms(d) + hours_to_ms(h) + minutes_to_ms(m)+
    seconds_to_ms(s) + ms_to_ms(ms);
    result
}

pub fn get_time() -> Instant {
    let time: Instant = Instant::now();
    time
}

pub fn unique_sym_to_str(str1: &str, str2: &str) -> String {
    if str1.is_empty() {
        debug_eprintln_fileinfo!("unique_sym_to_str str1 is empty");
        return (*str2).to_string();
    } else if str2.is_empty() {
        debug_eprintln_fileinfo!("unique_sym_to_str str2 is empty");
        return (*str1).to_string();
    }
    let mut result: String;

    result = (*str1).to_string();

    for i in (*str2).chars() {
        let finds: Option<usize> = result.find(i);
        if finds.is_none() {
            result.push(i);
        }
    }

    result
}

//O(n*m)
pub fn remove_duplicate_chars_simple_nm(s: &str) -> String {
    if s.len() > 64 {
        debug_println_fileinfo!("remove_duplicate_chars_simple_nm s.len() = {}\nможет лучше использовать remove_duplicate_chars_simple_n?",
        s.len());
    }
    let mut result = String::new();

    for ch in s.chars() {
        if !result.contains(ch) {
            result.push(ch);
        }
    }
    result
}

//O(n)
pub fn remove_duplicate_chars_simple_n(s: &str) -> String {
    if s.len() < 64 {
        debug_println_fileinfo!("remove_duplicate_chars_simple_n s.len() = {}\nможет нужно remove_duplicate_chars_simple_nm?",
        s.len());
    }
    let mut result = HashSet::new();
    s.chars().filter(|&c| result.insert(c)).collect()
}

pub fn unique_sym_to_vec_str(strs: &[String]) -> String {
    let mut result = strs.first().map(|s| s.as_str()).unwrap_or("").to_string();
    if result == "" {
        debug_eprintln_fileinfo!("strs[0] is empty");
    }
    debug_println_fileinfo!("items unique_sym_to_vec_str:");
    for item in strs.to_vec().iter().skip(1) {
        debug_println!("{}", item);
        result = unique_sym_to_str(&result, &item);
    }
    result
}

//todo дописать для произвольного вектора

pub fn max_slice<T>(sl1: &[T], sl2: &[T]) -> usize {
    if sl1.len() > sl2.len() {
        sl1.len()
    } else {
        sl2.len()
    }
}

pub fn min_slice<T>(sl1: &[T], sl2: &[T]) -> usize {
    if sl1.len() < sl2.len() {
        sl1.len()
    } else {
        sl2.len()
    }
}

pub fn sum_slice_len<T: std::ops::Add>(sl1: &[T], sl2: &[T]) -> usize {
    sl1.len() + sl2.len()
}

pub fn unique_sym_to_vec<T>(sl1: &[T], sl2: &[T]) -> Vec<T>
where
    T: std::ops::Add + std::clone::Clone + std::cmp::PartialEq + std::default::Default,
{
    let mut result: Vec<T> = Vec::new();
    if sl1.is_empty() {
        return sl2.to_vec();
    } else if sl2.is_empty() {
        return sl1.to_vec();
    }
    if max_slice(sl1, sl2) < 2 * min_slice(sl1, sl2) {
        //типо чтоб не создавать большой каписити если разница огромная
        result.reserve(sum_slice_len(sl1, sl2));
    } else {
        debug_println_fileinfo!(
            "sl1 len = {}\tsl2 len = {}\n[может стоить Распараллелить?)]",
            sl1.len(),
            sl2.len()
        );
    }

    result = (*sl1).to_vec();
    let mut last_len: usize = result.len();
    for item in sl2.iter() {
        let finds: Option<&T> = result.iter().find(|&x| x == item);
        if finds.is_none() {
            result.push(item.clone());
            last_len += 1;
        }
    }
    result.resize(last_len, T::default());
    result
}

pub fn convert_vec_to_owned<T>(vec: Vec<&T>) -> Vec<T::Owned>
where
    T: std::cmp::PartialEq + ToOwned + ?Sized,
{
    let result: Vec<<T as ToOwned>::Owned> = vec.into_iter().map(|t| t.to_owned()).collect();
    result
}
//надо для clone
//для строк можно проще с asref сделать fn для этого

//todo нужно для tinyvec?
//todo для слайца слайцов сделать
