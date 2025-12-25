use std::fmt::format;
use std::{cell::Cell, ops::Mul, u64::MAX};

use crate::abstract__tokinezer::{self, *};
use crate::str_utils::safe_remove_chars;
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

// более низкоуровенвый аналог Tokinezed_Error
#[derive(Clone)]
pub enum TokinezedErrorLow {
    NoneErr,
    SkipSymbolErr(String),
    SkipConstructionErr(String),
}

impl Default for TokinezedErrorLow {
    fn default() -> Self {
        Self::NoneErr
    }
}

pub fn skip_symbol_abstract_parse_value(
    str: &str,
    index: &mut usize,
    symbol_list: &str,
    err_cor: bool,
    file: String,
) -> Option<AbstractParseValue<char, TokinezedErrorLow>> {
    let mut err: Error<TokinezedErrorLow> = Error::default();
    let pos: Pos = Pos::default();
    let skip_res: bool = skip_symbol(str, index, symbol_list.to_string());
    let mut ret: AbstractParseValue<char, TokinezedErrorLow> = Default::default();
    let convert_res: Result<Pos, ()> =
        Pos::conver_to_pos(*index, str, Some(symbol_list.to_string()));
    if let Ok(ok) = convert_res {
        ret.set_pos(ok, err_cor);
    } else {
        return None;
    }
    if !skip_res {
        let cur_sym: Option<char> = str.chars().nth(*index);
        if cur_sym.is_some() {
            let cur_sym_unwrap: char = unsafe { cur_sym.unwrap_unchecked() };
            let msg: String = format!("not skip sym curr sym: {}", cur_sym_unwrap);
            err.set(
                pos,
                msg,
                file,
                TokinezedErrorLow::SkipConstructionErr(format!(
                    "symbol_list: {} cur_sym: {}",
                    symbol_list, cur_sym_unwrap
                )),
            );
            ret.set_err(err);
        }
    } else {
        let cur_sym: Option<char> = str.chars().nth(*index);
        let cur_sym_unwrap: char = unsafe { cur_sym.unwrap_unchecked() };
        ret.set_val(cur_sym_unwrap);
    }
    Some(ret)
}

//None это когда неудачный skip + неудачное преобразование в Pos
pub fn skip_construction_abstract_parse_value(
    str: &str,
    index: &mut usize,
    ignore_symbol_list: &str,
    construction: &str,
    skip_construct: &mut Construction,
    file: &str,
) -> Option<AbstractParseValue<String, TokinezedErrorLow>> {
    let mut start_find: bool = false;
    let mut err: Error<TokinezedErrorLow> = Error::default();
    let mut ret_abstract_parse_value: AbstractParseValue<String, TokinezedErrorLow> =
        Default::default();
    let mut give_sym_err: Option<String> = None;
    let mut iter: usize = 0;
    let len_str: usize = str.len();
    let len_construction: usize = construction.len();
    let mut chars_collision: String = Default::default();
    let option_collision_constuction: Option<Vec<usize>> =
        check_construction(ignore_symbol_list, construction, &mut chars_collision);
    let mut collision_construction: Vec<usize> = Vec::new();
    let mut fix_construction: String = String::default();
    //chars_collision.ch
    match option_collision_constuction {
        None => {}
        Some(x) => {
            collision_construction = x;
            debug_println_fileinfo!("Warning:collision syms {}", chars_collision);
        }
    }
    if !collision_construction.is_empty() {
        fix_construction = safe_remove_chars(construction, &collision_construction);
        if len_str < fix_construction.len() {
            return None;
        }
    } else {
        if len_str < len_construction {
            return None;
        }
    }
    crate::debug_println!("\n\n\n");
    loop {
        let skiping: Option<AbstractParseValue<char, TokinezedErrorLow>> =
            skip_symbol_abstract_parse_value(
                str,
                index,
                ignore_symbol_list,
                true,
                file.to_string(),
            );
        if skiping.is_some() {
            let skiping_unwrap: AbstractParseValue<char, TokinezedErrorLow> =
                unsafe { skiping.unwrap_unchecked() };
            if skiping_unwrap.is_val_some()
            //todo нужно соглосовать с collision_construction
            {
                if *index > len_str - 1 {
                    return None;
                }
                let option_str: Option<char> = get_symbol(str, *index);
                let mut give_sym_str: char = 'a';
                match option_str {
                    None => {} //невзможно из за проверки переполнения
                    Some(x) => {
                        give_sym_str = x;
                        if give_sym_err.clone().is_none() {
                            give_sym_err = Some("".to_string());
                        }
                        unsafe {
                            give_sym_err.as_mut().unwrap_unchecked().push(x); //rust если написть give_sym_err.unwrap_unchecked().push(x);
                        }
                    }
                }
                let option_construction: Option<char> = get_symbol(&fix_construction, iter);
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
                        //тут true в обычной skip_construcion
                        let give_sym_str_unwrap: String =
                            unsafe { give_sym_err.unwrap_unchecked() };
                        ret_abstract_parse_value.set_val(give_sym_str_unwrap);
                        let pos_construction: Result<Pos, ()> =
                            Pos::conver_to_pos(*index, str, None);
                        let unwrap_pos_construction: Pos =
                            unsafe { pos_construction.unwrap_unchecked() };
                        ret_abstract_parse_value.set_pos(unwrap_pos_construction, true);
                        return Some(ret_abstract_parse_value);
                    }
                } else {
                    //если чо в одном месте я просто unwrap_unchecked а в другом проверяю это потому что если ошибка то
                    //возможно она еще по причине например индекс больше cfg.len()
                    //а если удачно то ошибок не будет
                    let errpos: Result<Pos, ()> = Pos::conver_to_pos(*index, str, None);
                    if errpos.is_ok() {
                        let unwrap_errpos: Pos = unsafe { errpos.unwrap_unchecked() };
                        let msg_err: String = format!(
                            "Error skip construction: give_sym_construction ≠ give_sym_str"
                        );
                        let give_sym_err_unwrap: String =
                            unsafe { give_sym_err.unwrap_unchecked() };
                        let skip_construcion_msg: String = format!(
                            "give_char: {} construction_char: {}\ngive_sym_err: {} construction: {} index: {}",
                            give_sym_str,
                            give_sym_construction,
                            give_sym_err_unwrap,
                            construction,
                            *index
                        );
                        err.set(
                            unwrap_errpos,
                            msg_err,
                            file.to_string(),
                            TokinezedErrorLow::SkipConstructionErr(skip_construcion_msg),
                        );
                        return Some(ret_abstract_parse_value);
                    } else {
                        return None;
                    }
                }
                *index += 1;
                iter += 1;
            }

            if start_find && skip_construct.monolit {
                let errpos: Result<Pos, ()> = Pos::conver_to_pos(*index, str, None);
                if errpos.is_ok() {
                    let unwrap_errpos: Pos = unsafe { errpos.unwrap_unchecked() };
                    let msg_err: String = format!("Error skip construction: no monolit");
                    let give_sym_err_unwrap: String = unsafe { give_sym_err.unwrap_unchecked() };
                    let skip_construcion_msg: String =
                        format!("give_sym_err: {}  index: {}", give_sym_err_unwrap, *index);
                    err.set(
                        unwrap_errpos,
                        msg_err,
                        file.to_string(),
                        TokinezedErrorLow::SkipConstructionErr(skip_construcion_msg),
                    );
                    ret_abstract_parse_value.set_err(err);
                    return Some(ret_abstract_parse_value);
                } else {
                    return None;
                }
            }
        } else {
            return None;
        }
    }
}

//todo
fn check_construction(
    ignore_symbol_list: &str,
    construction: &str,
    collision_chars: &mut String,
) -> Option<Vec<usize>> {
    let mut idx: usize = 0;
    let mut collision: Vec<usize> = Vec::new();
    for items in construction.chars().enumerate() {
        let (s_i, s): (usize, char) = items;
        for c in ignore_symbol_list.chars() {
            if c == s {
                collision_chars.push(c);
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

#[derive(Clone)]
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
    let mut collision_chars: String = Default::default();
    let len_construction: usize = construction.len();
    let option_collision_constuction: Option<Vec<usize>> =
        check_construction(ignore_symbol_list, construction, &mut collision_chars);
    let mut collision_construction: Vec<usize> = Vec::new();
    match option_collision_constuction {
        None => {}
        Some(x) => {
            collision_construction = x;
        }
    }
    debug_println!(
        "Warning: collision_construction not empty {}",
        collision_chars
    );
    let mut fix_construction: String = construction.to_string();
    if !collision_construction.is_empty() {
        fix_construction = safe_remove_chars(construction, &collision_construction);

        if len_str < fix_construction.len() {
            return false;
        }
    } else {
        if len_str < len_construction {
            return false;
        }
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
            let option_construction: Option<char> = get_symbol(&fix_construction, iter);
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
