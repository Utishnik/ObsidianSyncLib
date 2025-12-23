use std::char;

use crate::debug_eprintln_fileinfo;
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
        if item.index < len{
            let idx: usize = item.index;
            let byte_pos: usize = 
            unsafe {
                 str
                .char_indices()
                .nth(idx)
                .map(|(pos, _)| pos)
                .unwrap_unchecked()
            };
            result.insert(byte_pos, item.char_);
        }
        else{
            result.push(item.char_);
        }
    }

    result
}

pub fn safe_insert_and_remove_chars(str: &str,chars_insert: &[InsertChar],chars_remove: &[usize]){
    let mut chars_remove_correct: Vec<usize>;
    for item in chars_remove{
        let mut shift: usize = 0;
        let _ = chars_insert.iter().filter(|&x| x.index < *item).map(|_| shift+=1);
    }
}