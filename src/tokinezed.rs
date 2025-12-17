use std::{cell::Cell, ops::Mul, u64::MAX};

use crate::{debug, debug_println, debug_println_fileinfo};

pub static MAX_TOKEN_CAPACITY: usize = 1000000;
pub static CAPACITY_UP_SIZE: usize = 64;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Email,
    AccTok,
    RemoteRepAddr,
    SetVal,
    PathObsidian,
    TimeCommit,
    TextCommit,
    UserName,
    IteratorStart,
    IteratorEnd,
}

impl Token {
    pub fn as_str(&self) -> &'static str {
        match self {
            Token::Email => "Email",
            Token::AccTok => "Token",
            Token::UserName => "UserName",
            Token::RemoteRepAddr => "Remote",
            Token::SetVal => "=",
            Token::PathObsidian => "Path",
            Token::TimeCommit => "Time",
            Token::TextCommit => "Text",
            Token::IteratorStart => "{{",
            Token::IteratorEnd => "}}",
        }
    }
}

pub fn get_symbol(str: &str, index: usize) -> Option<char> {
    str.chars().nth(index)
}

pub fn skip_symbol(str: &str, index: &mut usize, symbol_list: String) -> bool {
    let chr: Option<char> = get_symbol(str, *index);
    let value: char = match chr {
        None => {
            return false;
        }
        Some(x) => x,
    };

    for s in symbol_list.chars() {
        if value == s {
            *index += 1;
            return true;
        }
    }
    false
}

fn check_construction(ignore_symbol_list: &str, construction: &str) -> Option<Vec<usize>> {
    let mut idx: usize = 0;
    let mut collision: Vec<usize> = Vec::new();
    for items in construction.chars().enumerate() {
        let (s_i, s): (usize, char) = items;
        for c in ignore_symbol_list.chars() {
            if c == s {
                collision[idx] = s_i;
                idx += 1;
            }
        }
    }
    if collision.is_empty() {
        None
    } else {
        Some(collision)
    }
}

pub struct Construction {
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub monolit: bool,
}

impl Default for Construction {
    fn default() -> Self {
        Self {
            start: None,
            end: None,
            monolit: false,
        }
    }
}

impl Construction {
    pub fn get(&self) -> Self {
        Self {
            start: self.start,
            end: self.end,
            monolit: self.monolit,
        }
    }
    pub fn reset(&mut self) {
        self.start = None;
        self.end = None;
    }
    pub fn check_none(&self) -> bool {
        let start_option: Option<usize> = self.start;
        let end_option: Option<usize> = self.end;
        let mut start_none: bool = false;
        let mut end_none: bool = false;
        match start_option {
            None => start_none = true,
            Some(_) => {}
        }

        if end_option == None {
            end_none = true
        }
        !start_none && !end_none
    }
}

//todo функция скип символом пока не дойдет до определенного
pub fn skip_construction(
    str: &str,
    index: &mut usize,
    ignore_symbol_list: &str,
    construction: &str,
    skip_construct: &mut Construction,
) -> bool {
    let mut start_find: bool = false;
    let mut iter: usize = 0;
    let len_str: usize = str.len();
    let len_construction: usize = construction.len();
    let option_collision_constuction: Option<Vec<usize>> =
        check_construction(ignore_symbol_list, construction);
    let mut collision_construction: Vec<usize>;
    match option_collision_constuction {
        None => {}
        Some(x) => {
            collision_construction = x;
        }
    }
    if len_str < len_construction {
        return false;
    }
    crate::debug_println!("\n\n\n");
    loop {
        if !skip_symbol(str, index, ignore_symbol_list.to_string())
        //todo нужно соглосовать с collision_construction
        {
            if *index > len_str - 1 {
                return false;
            }
            let option_str: Option<char> = get_symbol(str, *index);
            let mut give_sym_str: char = 'a';
            match option_str {
                None => {} //невзможно из за проверки переполнения
                Some(x) => {
                    give_sym_str = x;
                }
            }
            let option_construction: Option<char> = get_symbol(&construction, iter);
            debug_println_fileinfo!("iter:   {}     index:    {}", iter, index);
            let mut give_sym_construction: char = 'b';
            match option_construction {
                None => {} //невозможно из за проверки переполнения
                Some(x) => {
                    give_sym_construction = x;
                }
            }

            debug_println!(
                "construct: {} src: {}  index: {}",
                give_sym_construction,
                give_sym_str,
                iter
            );
            if give_sym_construction == give_sym_str {
                if !start_find {
                    start_find = true;
                    skip_construct.start = Some(*index);
                }
                if iter == len_construction - 1 {
                    skip_construct.end = Some(*index);
                    return true;
                }
            } else {
                return false;
            }
            *index += 1;
            iter += 1;
        }

        if start_find && skip_construct.monolit {
            return false;
        }
    }
}

pub struct Token_String {
    pub tok_start: usize,
    pub tok_end: usize,
    pub tok_val: String,
}

#[derive(Clone)]
pub struct TokenStruct {
    pub tok_values: Vec<String>,
    pub tok_lines_number: Vec<u64>,
}

impl TokenStruct {
    pub fn new(size: usize) -> Self {
        Self {
            tok_values: vec![String::new(); size],
            tok_lines_number: Vec::<u64>::with_capacity(size),
        }
    }

    pub fn get_size(&self) -> usize {
        self.tok_values.len()
    }

    pub fn add_ch(&mut self, idx: usize, ch: char) -> Result<(), String> {
        let empty_state: bool = self.tok_values[idx].is_empty();
        let size_vec: usize = self.tok_values.len();
        if size_vec < idx {
            debug_println_fileinfo!("size_vec: {}  idx: {}", size_vec, idx);
            return Err("size_vec overflow".to_string());
        }
        if empty_state {
            debug_println_fileinfo!("add_ch empty!");
            return Err("add_ch empty!".to_string());
        } else {
            self.tok_values[idx].push(ch);
        }
        Ok(())
    }

    fn add_ln_num(&mut self, idx: usize, line: u64) -> Result<(), String> {
        let size_vec: usize = self.tok_lines_number.len();
        let overflow_state: bool = idx >= size_vec;

        if overflow_state {
            debug_println_fileinfo!("overflow size_vec: {}  idx: {}", size_vec, idx);
            return Err("overflow".to_string());
        } else {
            debug_println_fileinfo!("not overflow idx:  {}  line:  {}", idx, line);
            self.tok_lines_number[idx] = line;
        }
        Ok(())
    }

    pub fn safe_add_ln_num(&mut self, idx: usize, line: u64) -> Result<(), String> {
        let result: Result<(), String> = self.add_ln_num(idx, line);
        if result.is_err() {
            debug_println_fileinfo!("[safe_add_ln_num capacity up] idx = {}", idx);
            let mut capacity: usize = self.tok_lines_number.capacity();
            if capacity < CAPACITY_UP_SIZE {
                capacity += CAPACITY_UP_SIZE;
            }
            let float_capacity: f64 = ((capacity) as f64) * 0.75;
            let usize_float_capacity: usize = float_capacity.round() as usize;
            debug_println!("[usize_float_capacity: {}]", usize_float_capacity);
            let len_vec: usize = self.tok_lines_number.len();
            debug_println!("[len vec = {}]", len_vec);
            if len_vec <= usize_float_capacity
            // todo эту проверку надо вывести выше чтоб даже если не overflow
            {
                if capacity > MAX_TOKEN_CAPACITY {
                    let err_res: String = format!(
                        "overflow capacity {}  MAX_TOKEN_CAPACITY: {}",
                        capacity * 2,
                        MAX_TOKEN_CAPACITY
                    );
                    debug_println_fileinfo!("{}", err_res);
                    return Err(err_res);
                }
                let new_capacity: usize = capacity * 2;
                debug_println_fileinfo!("[new capacity = {}]", new_capacity);
                self.tok_lines_number.resize(new_capacity, 0);
            }
        }
        Ok(())
    }
}
