use std::default;

use crate::debug_eprintln_fileinfo;
use crate::debug_println;
use crate::debug_println_fileinfo;

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

//todo через where
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

//todo нужно для tinyvec?
//todo для слайца слайцов сделать
