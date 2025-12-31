use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;
use std::time::Instant;

use crate::debug_eprintln_fileinfo;
use crate::debug_println;
use crate::debug_println_fileinfo;
use crate::number_utils::DivisionError;
use crate::number_utils::safe_divide_with_remainder;

const MILLIS_PER_MILLIS: u128 = 1;
const MILLIS_PER_SECOND: u128 = 1000;
const MILLIS_PER_MINUTE: u128 = 60 * MILLIS_PER_SECOND; // 60,000
const MILLIS_PER_HOUR: u128 = 60 * MILLIS_PER_MINUTE; // 3,600,000
const MILLIS_PER_DAY: u128 = 24 * MILLIS_PER_HOUR; // 86,400,000

const MAX_MILLIS: u128 = 1000;
const MAX_SECOND: u128 = 60; //59.9999... -> 60
const MAX_MINUTE: u128 = 60;
const MAX_HOUR: u128 = 60;

pub const DEFAULT_HEURISTICS_VAL: f64 = 1.0 / 100.0;

#[derive(PartialEq, Clone, Ord, Eq, Default)]
pub struct TimePoint {
    days: u128,
    hours: u128,
    minutes: u128,
    seconds: u128,
    miliseconds: u128,
}

pub fn substr_by_char_start_idx_owned(str: &str, start_idx: usize) -> String {
    let start_byte: &str = str
        .char_indices()
        .nth(start_idx)
        .map(|(_i, _)| &str[start_idx..])
        .unwrap_or("");
    let owned: String = start_byte.to_string();
    owned
}

pub fn substr_by_char_end_idx_owned(str: &str, end_idx: usize) -> String {
    let end_byte: &str = str
        .char_indices()
        .nth(end_idx)
        .map(|(_i, _)| &str[..end_idx])
        .unwrap_or("");
    let owned: String = end_byte.to_string();
    owned
}

pub fn substr_by_char_start_idx_ref(str: &str, start_idx: usize) -> &str {
    str.char_indices()
        .nth(start_idx)
        .map(|(_i, _)| &str[start_idx..])
        .unwrap_or("")
}

pub fn substr_by_char_end_idx_ref(str: &str, end_idx: usize) -> &str {
    str.char_indices()
        .nth(end_idx)
        .map(|(_i, _)| &str[..end_idx])
        .unwrap_or("")
}

pub fn substr_by_char_end_start_idx_owned(
    str: &str,
    start_idx: usize,
    end_idx: usize,
) -> Option<String> {
    let substr: String =
        substr_by_char_start_idx_owned(substr_by_char_end_idx_ref(str, start_idx), end_idx);
    let mut result: Option<String> = None;
    if substr != "" {
        result = Some(substr);
    }
    result
}

pub fn substr_by_char_end_start_idx_ref(
    str: &str,
    start_idx: usize,
    end_idx: usize,
) -> Option<&str> {
    let substr: &str =
        substr_by_char_start_idx_ref(substr_by_char_end_idx_ref(str, start_idx), end_idx);
    let mut result: Option<&str> = None;
    if substr != "" {
        result = Some(substr);
    }
    result
}

#[derive(Clone, Debug)]
pub struct TimePointErr {
    err_type: DivisionError,
    err_msg: String,
}

impl TimePointErr {
    pub fn get_err_type(&self) -> DivisionError {
        self.err_type.clone()
    }
    pub fn get_err_msg(&self) -> String {
        self.err_msg.clone()
    }
}

impl fmt::Display for TimePointErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.err_type {
            DivisionError::DivisionByZero => writeln!(f, "Error Type: DivisionByZero")?,
            DivisionError::Overflow => writeln!(f, "Error Type: Overflow")?,
        };
        writeln!(f, "Error Msg: {}", self.err_msg)
    }
}

#[derive(Clone, Default)]
pub struct Discharge {
    new_val: Option<u128>,
    delta_next: Option<u128>,
}

impl Discharge {
    pub fn init(new_val: Option<u128>, delta_next: Option<u128>) -> Self {
        Self {
            new_val,
            delta_next,
        }
    }
    pub fn init_some(new_val: u128, delta_next: u128) -> Self {
        let option_new_val: Option<u128> = Some(new_val);
        let option_delta_next: Option<u128> = Some(delta_next);
        Self {
            new_val: option_new_val,
            delta_next: option_delta_next,
        }
    }
    fn set(&mut self, new_new_val: Option<u128>, new_delta_next: Option<u128>) {
        self.delta_next = new_delta_next;
        self.new_val = new_new_val;
    }
}

pub fn transf_discharge(val: u128, max_val: u128) -> Result<Discharge, DivisionError> {
    let mut new_val: u128 = 0;
    let mut delta: u128 = 0;
    let mut result: Discharge = Discharge::default();
    //delta.checked_rem(rhs)
    if val > max_val {
        let div: Result<(u128, u128), crate::number_utils::DivisionError> =
            safe_divide_with_remainder(val, max_val);
        if let Err(err) = div {
            debug_eprintln_fileinfo!("type {}", err);
            return Err(err);
        } else if let Ok(ok) = div {
            new_val = ok.1; //safe
            delta = ok.0;
        }
    }
    result.set(Some(new_val), Some(delta));
    Ok(result)
}

impl TimePoint {
    pub fn new(
        days: u128,
        hours: u128,
        minutes: u128,
        seconds: u128,
        miliseconds: u128,
    ) -> Result<Self, (DivisionError, String)> {
        let errhand: RefCell<Option<DivisionError>> = RefCell::new(None);
        let all_transf_discharge =
            |val: u128, max_val, transf_val: &mut Discharge| -> Result<(), DivisionError> {
                let transfm_res: Result<Discharge, DivisionError> = transf_discharge(val, max_val);
                if let Ok(ok) = transfm_res {
                    if ok.new_val.unwrap_or(miliseconds) != miliseconds
                        || ok.delta_next.unwrap_or(0) != 0
                    {
                        *transf_val = ok;
                    }
                } else if let Err(err) = transfm_res {
                    *errhand.borrow_mut() = Some(err);
                    return Err(errhand.borrow().clone().unwrap());
                }
                Ok(())
            };

        let mut transf_ms: Discharge = Discharge {
            new_val: None,
            delta_next: None,
        };

        let mut transf_s: Discharge = Discharge {
            new_val: None,
            delta_next: None,
        };

        let mut transf_m: Discharge = Discharge {
            new_val: None,
            delta_next: None,
        };

        let mut transf_h: Discharge = Discharge {
            new_val: None,
            delta_next: None,
        };

        let mut transf_d: Discharge = Discharge {
            new_val: None,
            delta_next: None,
        };

        let msec_res: Result<(), DivisionError> =
            all_transf_discharge(miliseconds, MAX_MILLIS, &mut transf_ms);

        if let Err(err) = msec_res {
            return Err((err, "msec_res".to_string()));
        }

        let sec_res: Result<(), DivisionError> = all_transf_discharge(
            seconds + transf_ms.delta_next.unwrap_or(0),
            MAX_SECOND,
            &mut transf_s,
        );

        if let Err(err) = sec_res {
            return Err((err, "sec_res".to_string()));
        }

        let m_res: Result<(), DivisionError> = all_transf_discharge(
            minutes + transf_s.delta_next.unwrap_or(0),
            MAX_MINUTE,
            &mut transf_m,
        );

        if let Err(err) = m_res {
            return Err((err, "m_res".to_string()));
        }

        let h_res: Result<(), DivisionError> = all_transf_discharge(
            hours + transf_m.delta_next.unwrap_or(0),
            MAX_HOUR,
            &mut transf_h,
        );

        if let Err(err) = h_res {
            return Err((err, "h_res".to_string()));
        }

        let d_res: Result<(), DivisionError> = all_transf_discharge(
            days + transf_m.delta_next.unwrap_or(0),
            u32::MAX as u128,
            &mut transf_d,
        );

        if let Err(err) = d_res {
            return Err((err, "d_res".to_string()));
        }

        let result: TimePoint = Self {
            days,
            hours,
            minutes,
            seconds,
            miliseconds,
        };

        //if miliseconds >
        Ok(result)
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

impl PartialOrd for TimePoint {
    fn ge(&self, other: &Self) -> bool {
        let self_mils: u128 = self.time_point_to_miliseconds();
        let other_self_mils: u128 = other.time_point_to_miliseconds();
        self_mils >= other_self_mils
    }
    fn gt(&self, other: &Self) -> bool {
        let self_mils: u128 = self.time_point_to_miliseconds();
        let other_self_mils: u128 = other.time_point_to_miliseconds();
        self_mils > other_self_mils
    }
    fn le(&self, other: &Self) -> bool {
        let self_mils: u128 = self.time_point_to_miliseconds();
        let other_self_mils: u128 = other.time_point_to_miliseconds();
        self_mils <= other_self_mils
    }
    fn lt(&self, other: &Self) -> bool {
        let self_mils: u128 = self.time_point_to_miliseconds();
        let other_self_mils: u128 = other.time_point_to_miliseconds();
        self_mils < other_self_mils
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
        debug_println_fileinfo!(
            "remove_duplicate_chars_simple_nm s.len() = {}\nможет лучше использовать remove_duplicate_chars_simple_n?",
            s.len()
        );
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
        debug_println_fileinfo!(
            "remove_duplicate_chars_simple_n s.len() = {}\nможет нужно remove_duplicate_chars_simple_nm?",
            s.len()
        );
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

pub fn compute_clean_capasity<T>(vec: &mut Vec<T>) -> usize {
    let not_init_vec_slice: &[std::mem::MaybeUninit<T>] = vec.spare_capacity_mut();
    let len_not_init_vec_slice: usize = not_init_vec_slice.len();
    let new_len: usize = vec.len() - len_not_init_vec_slice;
    new_len
}

pub fn set_new_len<T>(vec: &mut Vec<T>, compute_new_len: usize) {
    unsafe {
        vec.set_len(compute_new_len);
    }
}

pub fn clean_capasity<T>(vec: &mut Vec<T>) {
    let len: usize = compute_clean_capasity(vec);
    set_new_len(vec, len);
}

pub fn clean_capasity_heuristics<T>(vec: &mut Vec<T>, heuristics_val: f64) {
    let len: usize = compute_clean_capasity(vec);
    if heuristics_val > 1.0 {
        return;
    }
    if vec.len() - len < (((vec.len() as f64) * heuristics_val) as usize) {
        set_new_len(vec, len);
    }
}
