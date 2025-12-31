use crate::ParseExprError;
use crate::black_list_iterator::*;
use crate::debug_eprintln;
use crate::clear_console;
use crate::debug_eprintln_fileinfo;
use crate::debug_println;
use crate::reset_color_eprint;
use crate::reset_color_print;
use crate::set_color_eprint;
use crate::set_color_print;
use crate::splitt_b_space;
use crate::str_utils::gen_decimal_digits;
use crate::tokinezed;
use crate::tokinezed::skip_symbol_abstract_parse_value;
use crate::tokinezed::*;
use crate::utils::DEFAULT_HEURISTICS_VAL;
use crate::utils::TimePointErr;
use crate::utils::clean_capasity_heuristics;
use core::fmt;
use std::cell::LazyCell;
use std::default;
use std::error::Error as OtherError;
use std::ffi::os_str::Display;
use std::marker::PhantomData;
use tinyvec;

static CAPASITY_MIN: usize = 10;

thread_local! {
    pub static VAR_CHARS: LazyCell<String> = LazyCell::new(||{
        let ignored_symbols: String = AsciiSymbol::new("\"<>{}[],./*%!?-+()".to_string()).collect();
        ignored_symbols
    });
}

#[derive(Clone, Debug, Default)]
pub struct Pos {
    col: usize,
    line: usize,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Col: {}\tLine: {}", self.col, self.line)
    }
}

impl Pos {
    pub fn set(&mut self, col: usize, line: usize) {
        self.col = col;
        self.line = line;
    }
    pub fn get_col(&self) -> usize {
        self.col
    }
    pub fn get_line(&self) -> usize {
        self.line
    }
    pub fn conver_to_idx(&self, cfg: &str) -> Result<usize, ()> {
        let mut idx: usize = 0;
        let ret_res: Result<tokinezed::TokenStruct, ()> =
            splitt_b_space(cfg.to_string(), None, Some("\n".to_string()));
        if let Err(err) = ret_res {
            debug_eprintln_fileinfo!("conver_to_idx Err ret_res");
            return Err(err);
        } else if let Ok(ok) = ret_res {
            //надо суммировать длину каждой строки в векторе вплоть до нужной
            for item in ok.tok_values.iter().enumerate() {
                let (i, str) = item;
                let str_len: usize = str.len();
                if i != self.line {
                    idx += str_len;
                } else {
                    //нужно до конкретно col
                    idx += self.col - 1; //типо с нуля начинаем же
                }
            }
        }
        Ok(idx)
    }
    pub fn conver_to_pos(idx: usize, cfg: &str, transfers: Option<String>) -> Result<Pos, ()> {
        let mut ret_pos: Pos = Pos::default();
        let ret_res: Result<tokinezed::TokenStruct, ()>;
        if idx > cfg.len() {
            set_color_eprint(crate::Colors::Red);
            debug_eprintln_fileinfo!("idx > cfg.len()\tidx: {}  cfg.len(): {}", idx, cfg.len());
            reset_color_eprint();
            return Err(());
        }
        if transfers == None {
            ret_res = splitt_b_space(cfg.to_string(), None, Some("\n".to_string()));
        } else if unsafe { transfers.clone().unwrap_unchecked().is_empty() } {
            set_color_print(crate::Colors::Blue);
            debug_println!(
                "conver_to_pos transfers.is_empty() это случай только если одна строкой без переноса"
            );
            reset_color_print();
            ret_res = splitt_b_space(cfg.to_string(), None, None);
        } else {
            ret_res = splitt_b_space(cfg.to_string(), None, unsafe {
                Some(transfers.clone().unwrap_unchecked())
            });
        }
        let mut it: usize = 0;
        if let Err(err) = ret_res {
            set_color_eprint(crate::Colors::Yellow);
            debug_eprintln!("conver_to_pos ret_res is error");
            reset_color_eprint();
            return Err(err);
        } else if let Ok(ok) = ret_res {
            for item in ok.tok_values.iter().enumerate() {
                let (i, str) = item;
                let str_len: usize = str.len();
                if it + str_len < idx {
                    it += str_len;
                } else {
                    ret_pos.set(idx - it, i);
                    break;
                }
            }
        }
        Ok(ret_pos)
    }
}

#[derive(Clone, Debug)]
pub struct Error<T>
where
    T: std::clone::Clone,
{
    pos: Pos,
    msg: String,
    file: String,
    err_type: T,
}

impl<T> fmt::Display for Error<T>
where
    T: std::clone::Clone + fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self.pos)?;
        writeln!(f, "{}", self.msg)?;
        writeln!(f, "{}", self.file)?;
        writeln!(f, "{}", self.err_type)?;
        Ok(())
    }
}

impl<T> Default for Error<T>
where
    T: std::clone::Clone + Default,
{
    fn default() -> Self {
        Self {
            pos: Pos::default(),
            msg: "".to_string(),
            file: "".to_string(),
            err_type: T::default(),
        }
    }
}

impl<T> Error<T>
where
    T: std::clone::Clone,
{
    pub fn set(&mut self, pos: Pos, msg: String, file: String, err_type: T) {
        self.err_type = err_type;
        self.file = file;
        self.msg = msg;
        self.pos = pos;
    }

    pub fn get(&self) -> Self {
        Self {
            pos: self.pos.clone(),
            msg: self.msg.clone(),
            file: self.file.clone(),
            err_type: self.err_type.clone(),
        }
    }
}

pub enum ParseIntError {
    Overflow(String),
    SkipError(String),
    Empty,
    None,
}

pub fn parse_int_value<T>(str: &str, index: &mut usize, file: &str) -> Result<T, ParseIntError>
where
    T: std::default::Default + std::str::FromStr,
{
    let check_number = |char: char| -> bool { char.is_ascii_digit() };
    let mut find_start: bool = false;
    let mut curr_str: String = String::default();
    let sym_list: String = gen_decimal_digits();
    loop {
        let skiping: Option<AbstractParseValue<char, tokinezed::TokinezedErrorLow>> =
            skip_symbol_abstract_parse_value(str, index, &sym_list, true, file.to_string());
        //как будто clone не blazing
        if let Some(val) = skiping
            && unsafe { check_number(val.val.unwrap_unchecked()) }
        {
            find_start = true;
            let unwrap_ch: char = unsafe { val.val.unwrap_unchecked() };
            curr_str.push(unwrap_ch);
        } else if !find_start {
            return Err(ParseIntError::SkipError(format!(
                "index: {}, curr_str: {}",
                index, curr_str
            )));
        } else {
            break;
        }
    }
    let ret_val: T = unsafe { curr_str.parse().unwrap_unchecked() };
    Ok(ret_val)
}

/*
todo
например надо спарсить время d: '',h: '' ...
и в d:'' например могут быть не конретно дни а милисекунды и это удобная
обертка для работы с этим
*/
pub trait AbstractValue {
    type Item;
    fn set(&mut self, item: Self::Item);
    fn get_ref(&self) -> Option<&Self::Item>;
    fn get_owned(&self) -> Option<Self::Item>;
    fn get_pos(&self) -> Option<Pos>;
    fn get_idx(&self) -> Option<usize>;
    fn size_of(&self) -> usize;
    fn size(&self) -> usize;
    fn parse_value(str: &str, index: &mut usize) -> Result<Self::Item, ParseExprError>;
}

impl AbstractValue for u128 {
    type Item = u128;
    fn set(&mut self, item: Self::Item) {
        *self = item;
    }
    fn get_ref(&self) -> Option<&Self::Item> {
        Some(self)
    }
    fn get_owned(&self) -> Option<Self::Item> {
        Some(*self)
    }
    fn get_pos(&self) -> Option<Pos> {
        None
    }
    fn get_idx(&self) -> Option<usize> {
        None
    }
    fn size_of(&self) -> usize {
        size_of::<u128>()
    }
    fn size(&self) -> usize {
        1
    }
    fn parse_value(str: &str, index: &mut usize) -> Result<u128, ParseExprError> {
        let ret_val: u128 = 0;

        Ok(ret_val)
    }
}

impl AbstractValue for char {
    type Item = char;
    fn set(&mut self, item: Self::Item) {
        *self = item;
    }
    fn get_ref(&self) -> Option<&Self::Item> {
        Some(self)
    }
    fn get_owned(&self) -> Option<Self::Item> {
        Some(*self)
    }
    fn get_pos(&self) -> Option<Pos> {
        None
    }
    fn get_idx(&self) -> Option<usize> {
        None
    }
    fn size_of(&self) -> usize {
        size_of::<char>()
    }
    fn size(&self) -> usize {
        1
    }
    fn parse_value(str: &str, index: &mut usize) -> Result<char, ParseExprError> {
        let ret_val = ' ';

        Ok(ret_val)
    }
}

impl AbstractValue for String {
    type Item = String;
    fn set(&mut self, item: Self::Item) {
        *self = item;
    }
    fn get_ref(&self) -> Option<&Self::Item> {
        Some(self)
    }
    fn get_owned(&self) -> Option<Self::Item> {
        Some(self.clone())
    }
    fn get_pos(&self) -> Option<Pos> {
        None
    }
    fn get_idx(&self) -> Option<usize> {
        None
    }
    fn size_of(&self) -> usize {
        size_of::<String>() * self.len()
    }
    fn size(&self) -> usize {
        self.len()
    }
    //сюда надо передать кусок строки и индекс для этого отдельный метод нужен
    #[inline]
    fn parse_value(str: &str, index: &mut usize) -> Result<String, ParseExprError> {
        let mut ret_val: String = String::default();
        let var_chars: String = VAR_CHARS.with(|x: &LazyCell<String>| x.as_str().to_string());

        loop {
            debug_println!("var_chars: {}", var_chars);
            let res_skip: Option<AbstractParseValue<char, tokinezed::TokinezedErrorLow>> =
                skip_symbol_abstract_parse_value(str, index, &var_chars, true, "".to_string());
            if res_skip.is_none() {
                debug_eprintln!("parse_value impl AbstractValue for String res skip is none");
                return Err(ParseExprError::None);
            }
            let unwrap_res_skip: AbstractParseValue<char, tokinezed::TokinezedErrorLow> =
                unsafe { res_skip.unwrap_unchecked() };
            if unwrap_res_skip.is_err() {
                let unwrap_res_skip_err: Error<tokinezed::TokinezedErrorLow> =
                    unsafe { unwrap_res_skip.err.unwrap_unchecked() };
                let err_msg: String = unwrap_res_skip_err.msg;
                let err_file: String = unwrap_res_skip_err.file;
                set_color_eprint(crate::Colors::Red);
                clear_console();
                debug_eprintln_fileinfo!(
                    "parse_value impl AbstractValue for String unwrap_res_skip.is_err() msg: {} file: {}",
                    err_msg,
                    err_file
                );
                reset_color_eprint();
                set_color_eprint(crate::Colors::Yellow);
                debug_eprintln!("error ret_val: {}",ret_val);
                reset_color_eprint();
                return Err(ParseExprError::KeyWord(format!(
                    "parse_value impl AbstractValue for String unwrap_res_skip.is_err() msg: {} file: {}",
                    err_msg, err_file
                )));
            }

            let unwrap_val: char = unsafe { unwrap_res_skip.val.unwrap_unchecked() };
            if unwrap_val.is_ascii_whitespace() {
                break;
            } else {
                ret_val.push(unwrap_val);
            }
        }

        Ok(ret_val)
    }
}

#[derive(Clone, Debug)]
pub struct AbstractParseValue<T, E>
where
    T: AbstractValue + std::clone::Clone,
    E: std::clone::Clone,
{
    pos: Pos,
    val: Option<T>,
    err: Option<Error<E>>,
}

#[derive(Clone)]
pub enum AbstractValType {
    Str,
    Num,
    Time,
}

#[derive(Clone, Default)]
pub struct AbstractParseExpr {
    pos: Option<Pos>,
    data: Vec<AbstractParseExpr>,
    val: Option<String>,
    val_type: Option<AbstractValType>,
}

//let var: num = 10;
//msg: "var = " + num
//пока без разбра let var ...
//предположим он уже равен 10
//msg <- expr первый проход еще без типа, "var = " - expr str,
//+ <- operation num -> expr значение которого 10 и тип num
//тоесть msg это выражения типа str равное сумме двух других
impl AbstractParseExpr {
    fn set_val(&mut self, val: String, val_type: AbstractValType, pos: Pos) {
        self.val = Some(val);
        self.val_type = Some(val_type);
        self.pos = Some(pos);
    }

    fn reset_val(&mut self) {
        self.val = None;
        self.val_type = None;
        self.pos = None;
    }

    fn push_data(&mut self, data: AbstractParseExpr) {
        self.data.push(data);
    }

    fn pop_data(&mut self) -> Option<AbstractParseExpr> {
        self.data.pop()
    }

    fn get_data(&self) -> &Vec<AbstractParseExpr> {
        &self.data
    }

    fn get_mut_data(&mut self) -> &mut Vec<AbstractParseExpr> {
        &mut self.data
    }
}

impl<T, E> fmt::Display for AbstractParseValue<T, E>
where
    T: AbstractValue + std::clone::Clone + std::fmt::Display,
    E: std::clone::Clone + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self.pos)?;
        if self.val.is_some() {
            writeln!(f, "{}", unsafe { self.val.clone().unwrap_unchecked() })?;
        } else {
            writeln!(f, "{}", unsafe { self.err.clone().unwrap_unchecked() })?;
        }
        Ok(())
    }
}

impl<T, E> Default for AbstractParseValue<T, E>
where
    T: AbstractValue + std::clone::Clone,
    E: std::clone::Clone,
{
    fn default() -> Self {
        Self {
            pos: Pos { col: 0, line: 0 },
            val: None,
            err: None,
        }
    }
}

impl<T, E> AbstractParseValue<T, E>
where
    T: AbstractValue + std::clone::Clone,
    E: std::clone::Clone,
{
    pub fn set_err(&mut self, err: Error<E>) {
        self.val = None;
        self.err = Some(err);
    }

    pub fn set_val(&mut self, val: T) {
        self.val = Some(val);
        self.err = None;
    }

    pub fn is_err(&self) -> bool {
        let err: Option<Error<E>> = self.err.clone();

        if err.is_none() { false } else { true }
    }
    pub fn is_val_some(&self) -> bool {
        let is_some: Option<T> = self.val.clone();

        if is_some.is_none() { false } else { true }
    }

    //err_cor отвечает за то что он автоматически сделает ошибко None
    pub fn set_pos(&mut self, pos: Pos, err_cor: bool) -> bool {
        let ret: bool = self.is_err();
        if ret == true && !err_cor {
            return false;
        }
        self.pos = pos;
        self.err = None;

        true
    }
}

pub struct ParseFnsVec<'a, F, T, E>
where
    F: FnOnce(&mut usize) -> AbstractParseValue<T, E> + std::clone::Clone + 'a,
    T: AbstractValue + std::clone::Clone,
    E: std::clone::Clone,
{
    fns: Vec<F>,
    len: usize,
    _marker: PhantomData<&'a ()>,
}

impl<'a, 'b, F, T, E> ParseFnsVec<'a, F, T, E>
where
    F: FnOnce(&mut usize) -> AbstractParseValue<T, E> + std::clone::Clone,
    T: AbstractValue + std::clone::Clone,
    E: std::clone::Clone,
{
    fn push_parse_fn(&mut self, val: F) {
        self.fns.push(val);
        self.len += 1;
    }
    fn len(&self) -> usize {
        self.len
    }
    fn iter(&'b self) -> std::slice::Iter<'b, F> {
        self.fns.iter()
    }
}

pub fn smart_abstract_multiconstruction_value_parse<F, T, E>(
    fns: ParseFnsVec<F, T, E>,
    idx: &mut usize,
) -> Vec<AbstractParseValue<T, E>>
where
    F: FnOnce(&mut usize) -> AbstractParseValue<T, E> + std::clone::Clone,
    T: AbstractValue + std::clone::Clone,
    E: std::clone::Clone,
{
    let len_fns: usize = fns.len();
    let mut result: Vec<AbstractParseValue<T, E>> = Vec::new();
    result.reserve(len_fns);
    for item in fns.iter() {
        result.push((item.clone())(idx));
    }
    result
}

pub fn get_abstract_value_in_vec<T, E>(
    vec: Vec<AbstractParseValue<T, E>>,
    idx: usize,
) -> Option<AbstractParseValue<T, E>>
where
    T: AbstractValue + std::clone::Clone,
    E: std::clone::Clone,
{
    let result: Option<&AbstractParseValue<T, E>> = vec.get(idx);
    let mut ret_res: Option<AbstractParseValue<T, E>> = None;
    if result.is_some() {
        ret_res = Some(unsafe { (result.unwrap_unchecked()).clone() });
    }
    ret_res
}

pub fn skip_value<T, E>(val: AbstractParseValue<T, E>) -> Result<(Pos, T), Error<E>>
where
    T: AbstractValue + std::clone::Clone,
    E: std::clone::Clone,
{
    let option_val: Option<T> = val.val.clone();
    if option_val.is_none() {
        return Err(unsafe { val.err.unwrap_unchecked() });
    }
    Ok((val.pos, unsafe { val.val.clone().unwrap_unchecked() }))
}

pub fn skip_values<T, E>(vals: &[AbstractParseValue<T, E>]) -> Vec<Result<(Pos, T), Error<E>>>
where
    T: AbstractValue + std::clone::Clone,
    E: std::clone::Clone,
{
    let mut ret: Vec<Result<(Pos, T), Error<E>>> = Vec::new();
    let len_vals: usize = vals.to_vec().len();
    ret.reserve(len_vals);
    for item in vals.to_vec().iter().enumerate() {
        let (i, v) = item;
        ret[i] = skip_value(v.clone());
    }
    ret
}

pub fn get_errs_skip_values<T, E>(
    vec: Vec<Result<(Pos, T), Error<E>>>,
) -> (Vec<Error<E>>, Vec<(Pos, T)>)
where
    T: AbstractValue + std::clone::Clone,
    E: std::clone::Clone,
{
    let vec_len: usize = vec.len();
    let mut err_vec: Vec<Error<E>> = Vec::new();
    let mut ok_vec: Vec<(Pos, T)> = Vec::new();
    let mid_len_vec: usize = vec_len / 4;
    let mut capasity_err_vec: usize = mid_len_vec;
    let mut capasity_ok_vec: usize = vec_len - mid_len_vec;
    if capasity_err_vec < CAPASITY_MIN {
        capasity_err_vec += CAPASITY_MIN;
    }
    if capasity_ok_vec < CAPASITY_MIN {
        capasity_ok_vec += CAPASITY_MIN;
    }
    err_vec.reserve(capasity_err_vec); //простая эвристика
    ok_vec.reserve(capasity_ok_vec);
    let mut idx_ok: usize = 0;
    let mut idx_err: usize = 0;
    for ret in vec.iter() {
        match ret {
            Ok(ok) => {
                if idx_err < capasity_ok_vec {
                    ok_vec[idx_ok] = ok.clone();
                } else {
                    capasity_ok_vec = vec_len;
                    ok_vec.reserve(capasity_ok_vec);
                    ok_vec[idx_ok] = ok.clone();
                }
                idx_ok += 1;
            }
            Err(err) => {
                if idx_err < capasity_err_vec {
                    err_vec[idx_err] = err.clone();
                } else {
                    capasity_err_vec *= 2;
                    err_vec.reserve(capasity_err_vec);
                    err_vec[idx_err] = err.clone();
                }
                idx_err += 1;
            }
        }
    }
    (err_vec, ok_vec)
    //todo собирать статистику в среднем какой reserve было и по ней делать а не просто / 4
}

pub fn get_errs_skip_values_clean<T, E>(
    vec: Vec<Result<(Pos, T), Error<E>>>,
) -> (Vec<Error<E>>, Vec<(Pos, T)>)
where
    T: AbstractValue + std::clone::Clone,
    E: std::clone::Clone,
{
    let mut res: (Vec<Error<E>>, Vec<(Pos, T)>) = get_errs_skip_values(vec);
    clean_capasity_heuristics(&mut res.0, DEFAULT_HEURISTICS_VAL);
    clean_capasity_heuristics(&mut res.1, DEFAULT_HEURISTICS_VAL);
    res
}

pub fn skip_value_index<T, E>(
    val: AbstractParseValue<T, E>,
    cfg: &str,
    file_path: String,
) -> Result<(usize, T), Error<E>>
where
    T: AbstractValue + std::clone::Clone + Default,
    E: std::clone::Clone + Default,
{
    let skip_val_res: Result<(Pos, T), Error<E>> = skip_value(val);
    let mut val_res: T = T::default();
    let mut match_res: usize = 0;
    if let Err(err) = skip_val_res {
        return Err(err);
    } else if let Ok(ok) = skip_val_res {
        let pos_ret: Pos = ok.0;
        val_res = ok.1;
        let conver_pos_to_usize: Result<usize, ()> = pos_ret.conver_to_idx(cfg);
        match_res = match conver_pos_to_usize {
            Ok(x) => x,
            Err(_) => {
                let err: Error<E> = Error {
                    pos: Pos::default(),
                    msg: "conver_to_idx err".to_string(),
                    file: file_path,
                    err_type: E::default(),
                };
                return Err(err);
            }
        };
    }
    Ok((match_res, val_res))
}
