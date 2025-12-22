use crate::debug_eprintln_fileinfo;
use crate::splitt_b_space;
use crate::tokinezed;
use crate::utils::clean_capasity_heuristics;
use crate::utils::DEFAULT_HEURISTICS_VAL;
use core::fmt;
use std::default;
use std::error::Error as OtherError;
use std::ffi::os_str::Display;
use std::marker::PhantomData;

static CAPASITY_MIN: usize = 10;

#[derive(Clone, Debug)]
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
        if transfers == None {
            ret_res = splitt_b_space(cfg.to_string(), None, Some("\n".to_string()));
        } else if unsafe { transfers.clone().unwrap_unchecked().is_empty() } {
            ret_res = splitt_b_space(cfg.to_string(), None, None);
        } else {
            ret_res = splitt_b_space(cfg.to_string(), None, unsafe {
                Some(transfers.clone().unwrap_unchecked())
            });
        }
        let mut it: usize = 0;
        if let Err(err) = ret_res {
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

impl Default for Pos {
    fn default() -> Self {
        Self { col: 0, line: 0 }
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

        if err.is_none() {
            false
        } else {
            true
        }
    }
    pub fn is_val_some(&self) -> bool {
        let is_some: Option<T> = self.val.clone();

        if is_some.is_none() {
            false
        } else {
            true
        }
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
    return Ok((match_res, val_res));
}
