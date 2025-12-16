use std::collections::HashSet;
use std::default;
use std::time::Instant;

use crate::debug_eprintln_fileinfo;
use crate::debug_println;
use crate::debug_println_fileinfo;

const MILLIS_PER_MILLIS: u128 = 1;
const MILLIS_PER_SECOND: u128 = 1000;
const MILLIS_PER_MINUTE: u128 = 60 * MILLIS_PER_SECOND; // 60,000
const MILLIS_PER_HOUR: u128 = 60 * MILLIS_PER_MINUTE; // 3,600,000
const MILLIS_PER_DAY: u128 = 24 * MILLIS_PER_HOUR; // 86,400,000

pub struct TimePoint {
    days: u128,
    hours: u128,
    minutes: u128,
    seconds: u128,
    miliseconds: u128,
}

impl TimePoint {
    pub fn new(days: u128, hours: u128, minutes: u128, seconds: u128, miliseconds: u128) -> Self {
        Self {
            days,
            hours,
            minutes,
            seconds,
            miliseconds,
        }
    }
    pub fn miliseconds_to_time_point(miliseconds: u128) -> Self {
        let mut clone_miliseconds: u128 = miliseconds;
        let days: u128 = clone_miliseconds / MILLIS_PER_DAY;
        clone_miliseconds -= days * MILLIS_PER_DAY;
        let hours: u128 = clone_miliseconds / MILLIS_PER_HOUR;
        clone_miliseconds -= hours * MILLIS_PER_HOUR;
        let minutes: u128 = clone_miliseconds / MILLIS_PER_MINUTE;
        clone_miliseconds -= minutes * MILLIS_PER_MINUTE;
        let seconds: u128 = clone_miliseconds / MILLIS_PER_SECOND;
        clone_miliseconds -= seconds * MILLIS_PER_SECOND;
        let miliseconds: u128 = clone_miliseconds / MILLIS_PER_MILLIS;
        clone_miliseconds -= miliseconds;
        debug_println!(
            "days {} hours {} minutes {} seconds {} mills {}",
            days,
            hours,
            minutes,
            seconds,
            miliseconds
        );
        Self {
            days,
            hours,
            minutes,
            seconds,
            miliseconds,
        }
    }
    pub fn time_point_to_miliseconds(&self) -> u128 {
        let mut result: u128 = 0;
        result += self.days * MILLIS_PER_DAY;
        result += self.hours * MILLIS_PER_HOUR;
        result += self.minutes * MILLIS_PER_MINUTE;
        result += self.seconds * MILLIS_PER_SECOND;
        result += self.miliseconds * MILLIS_PER_MILLIS;
        result
    }
}

fn days_to_ms(days: u128) -> u128 {
    days * MILLIS_PER_DAY
}
fn hours_to_ms(hours: u128) -> u128 {
    hours * MILLIS_PER_HOUR
}
fn minutes_to_ms(minutes: u128) -> u128 {
    minutes * MILLIS_PER_MINUTE
}
fn seconds_to_ms(seconds: u128) -> u128 {
    seconds * MILLIS_PER_SECOND
}
fn ms_to_ms(miliseconds: u128) -> u128 {
    miliseconds * MILLIS_PER_MILLIS
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
fn time_period_set(d: u128, h: u128, m: u128, s: u128, ms: u128) -> u128 {
    let result: u128 =
        days_to_ms(d) + hours_to_ms(h) + minutes_to_ms(m) + seconds_to_ms(s) + ms_to_ms(ms);
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
