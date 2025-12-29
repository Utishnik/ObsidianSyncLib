use std::char;

use crate::debug_eprintln_fileinfo;
use crate::debug_println_fileinfo;
use crate::display_vec;

#[derive(Clone)]
pub struct InsertChar {
    pub index: usize,
    pub char_: char,
}

pub fn contains_add_char_slice(chars: &[InsertChar], idx: usize) -> bool {
    chars.iter().any(|x| x.index == idx)
}

pub fn find_add_char_idx(chars: &[InsertChar], idx: usize) -> Option<char> {
    for item in chars {
        if item.index == idx {
            return Some(item.char_);
        }
    }
    None
}

pub fn safe_remove_chars(str: &str, char_indices: &[usize]) -> String {
    let len: usize = str.len();
    let mut incorrect_idx: Vec<usize> = Vec::new();
    let res: Vec<usize> = char_indices
        .iter()
        .filter(|&&x: &&usize| {
            incorrect_idx.push(x);
            x < len
        })
        .copied()
        .collect();
    debug_eprintln_fileinfo!(
        "indexes > str.len : {}",
        display_vec(&incorrect_idx, " ,".to_string())
    );
    str.char_indices()
        .enumerate()
        .filter(|(char_pos, (_, _))| !res.contains(char_pos))
        .map(|(_, (_, c))| c)
        .collect()
}

pub fn safe_insert_chars(str: &str, chars: &[InsertChar]) -> String {
    let mut sorted: Vec<InsertChar> = chars.to_vec();
    sorted.sort_by(|a: &InsertChar, b: &InsertChar| b.index.cmp(&a.index));
    let mut result: String = str.to_string();
    let len: usize = str.len();
    for item in sorted {
        if item.index < len {
            let idx: usize = item.index;
            let byte_pos: usize = unsafe {
                str.char_indices()
                    .nth(idx)
                    .map(|(pos, _)| pos)
                    .unwrap_unchecked()
            };
            result.insert(byte_pos, item.char_);
        } else {
            result.push(item.char_);
        }
    }

    result
}

pub fn safe_insert_and_remove_chars(
    str: &str,
    chars_insert: &[InsertChar],
    chars_remove: &mut [usize],
) -> String {
    let mut chars_remove_correct: Vec<usize> = Vec::new();
    let mut i: usize = 0;
    for item in chars_remove.iter() {
        //O(n)
        let mut shift: usize = 0;
        let _ = chars_insert
            .iter()
            .skip(i)
            .filter(|&x| x.index < *item)
            .map(|_| shift += 1);
        shift += *chars_remove_correct.get(i).unwrap_or(&0);
        chars_remove_correct.push(shift);
        i += 1;
    }
    display_vec(&chars_remove_correct, " ,".to_string());
    let insert_res: String = safe_insert_chars(str, chars_insert);
    debug_eprintln_fileinfo!("insert str: {}", insert_res);
    for item in chars_remove.iter_mut().enumerate() {
        let (i, v) = item;
        *v += chars_remove_correct[i];
    }
    let ret_res: String = safe_remove_chars(&insert_res, chars_remove);
    debug_eprintln_fileinfo!("insert str: {}", ret_res);
    ret_res
}

//todo можно ли тут применить формальный верефикатор для блокировки случаев создающий вечный цикл
pub fn gen_digit_or_number<F>(start: f64, end: f64, transform: F) -> String
where
    F: Fn(f64) -> f64,
{
    let mut ret: String = String::default();
    let mut i: f64 = start;
    while i < end {
        ret.push_str(&i.to_string());
        i = transform(i);
    }
    ret
}

pub fn gen_decimal_digits() -> String {
    let ret: String = gen_digit_or_number(0.0, 10.0, |x: f64| x + 1.0);
    ret
}

pub fn chunk_str_get(str: &str, start_idx: usize, end_idx: usize) -> String {
    let result: String = str.chars().skip(start_idx).take(end_idx).collect();
    result
}
