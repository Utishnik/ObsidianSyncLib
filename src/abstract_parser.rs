use super::debug::display_utils::display_vec;
use crate::debug_eprintln_fileinfo;
use crate::optional_error::OptionErr;
use crate::tokinezed::*;
use crate::{debug_println, utils::TimePoint};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use tinyvec::ArrayVec;

pub enum BinOp {
    Plus,
    Minus,
    Mul,
    Div,
}

impl BinOp {
    fn as_str(&self) -> &str {
        match self {
            BinOp::Plus => "+",
            BinOp::Minus => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
        }
    }
}

#[derive(Clone)]
enum NumberType {
    Float(f64),
    Int(i128),
    Uint(u128),
}

impl NumberType {
    fn set_float(&mut self, float_vaL: f64) {
        *self = self::NumberType::Float(float_vaL);
    }
    fn set_int(&mut self, int_val: i128) {
        *self = self::NumberType::Int(int_val);
    }
    fn set_uint(&mut self, uint_val: u128) {
        *self = self::NumberType::Uint(uint_val);
    }
}

#[derive(Clone)]
enum AbstractVarValue {
    Str(String),
    Num(NumberType),
    Time(TimePoint),
    None,
}

impl AbstractVarValue {
    fn set_str(&mut self, str: String) {
        *self = Self::Str(str);
    }
    fn set_number(&mut self, num: NumberType) {
        *self = Self::Num(num);
    }
    fn set_time(
        &mut self,
        time: TimePoint,
    ) -> crate::optional_error::OptionErr<(crate::number_utils::DivisionError, String)> {
        //fix Optinal Err
        let cmp_time: Result<TimePoint, (crate::number_utils::DivisionError, String)> =
            TimePoint::new(0, 0, 1, 0, 0);
        let mut cmp_time_unwrap: TimePoint = TimePoint::default(); //todo пофиксить Error E0381
        if let Ok(val) = cmp_time {
            cmp_time_unwrap = val;
        } else if let Err(err) = cmp_time {
            debug_println!(
                "время синхронизации меньше или равно 1 минуты может не нужно так часто?\ntodo сделать Display для TimePoint"
            );
            return crate::optional_error::OptionErr::Err(err);
        }
        if time <= cmp_time_unwrap {}
        *self = Self::Time(time);
        crate::optional_error::OptionErr::None
    }
}

#[derive(Clone)]
pub struct Var {
    value: AbstractVarValue,
    name: String,
}

pub static VARS: OnceLock<Arc<Mutex<Vec<Var>>>> = OnceLock::new();

fn validate_var_name<F>(tok: &String, var_symbol_black_list_pat: F) -> bool
where
    F: Fn(&String) -> bool,
{
    var_symbol_black_list_pat(tok)
}

fn symbols_pat(str: &String) -> bool {
    false
}

fn parse_decl_vars_in_tok_struct(tok_struct: &TokenStruct) -> OptionErr<String> {
    let mut iter_item: usize = 0;
    for item in &tok_struct.tok_values {
        if item == Token::Let.as_str() {
            let idx: usize = iter_item;
            let nxt_tok: Option<&String> = tok_struct.tok_values.get(idx + 1);
            if nxt_tok.is_none() {
                return OptionErr::Err("let .....".to_string());
            }
            let nxt_tok_unwrap: &String = unsafe { nxt_tok.unwrap_unchecked() };

            //в AbstractValue должна быть реализация
        }
        iter_item += 1;
    }
    OptionErr::None
}

pub fn parse_decl_vars() -> OptionErr<()> {
    let res: Result<&Arc<Mutex<TokenStruct>>, ()> = get_and_init_tokens();
    if res.is_err() {
        return OptionErr::Err(());
    }
    let unwrap_res: &Arc<Mutex<TokenStruct>> = unsafe { res.unwrap_unchecked() };
    let unwrap_lock: Result<
        std::sync::MutexGuard<'_, TokenStruct>,
        std::sync::PoisonError<std::sync::MutexGuard<'_, TokenStruct>>,
    > = unwrap_res.lock();
    match unwrap_lock {
        Ok(guard) => {
            parse_decl_vars_in_tok_struct(&guard);
        }
        Err(_) => {
            debug_eprintln_fileinfo!(
                "parse_decl_vars отравленный поток🤢; todo сделать восстановление"
            );
            return OptionErr::Err(());
        }
    }
    OptionErr::None
}

pub fn get_or_init_vars() -> Result<&'static Arc<Mutex<Vec<Var>>>, ()> {
    let res: &Arc<Mutex<Vec<Var>>> = VARS.get_or_init(|| Arc::new(Mutex::new(Vec::new())));
    let guard: Result<
        std::sync::MutexGuard<'_, Vec<Var>>,
        std::sync::PoisonError<std::sync::MutexGuard<'_, Vec<Var>>>,
    > = res.lock();

    if guard.is_err() {
        debug_eprintln_fileinfo!("отравленный поток🤢; todo сделать восстановление");
        return Err(());
    }
    Ok(res)
}

#[inline] //такие же как и оригинальный пуш атрибут
pub fn push_var(var: Var) {
    let pack_vars: Result<&Arc<Mutex<Vec<Var>>>, ()> = get_or_init_vars();
    let unwrap_vars: &Arc<Mutex<Vec<Var>>> = unsafe { pack_vars.unwrap_unchecked() };
    let mut guard: std::sync::MutexGuard<'_, Vec<Var>> =
        unsafe { unwrap_vars.lock().unwrap_unchecked() }; //обработка отравления на стороне get_or_init_vars
    guard.push(var);
}

#[inline]
pub fn get_var(index: usize) -> Option<Var> {
    let pack_vars: Result<&Arc<Mutex<Vec<Var>>>, ()> = get_or_init_vars();
    let unwrap_vars: &Arc<Mutex<Vec<Var>>> = unsafe { pack_vars.unwrap_unchecked() };
    let guard: std::sync::MutexGuard<'_, Vec<Var>> =
        unsafe { unwrap_vars.lock().unwrap_unchecked() };
    guard.get(index).cloned()
}

#[derive(Clone, Default)]
pub struct Slice {
    start_index: usize,
    end_index: usize,
}

impl Slice {
    fn set(&mut self, start_idx: usize, end_idx: usize) {
        self.start_index = start_idx;
        self.end_index = end_idx;
    }
    fn set_zero_dimensional(&mut self, index: usize) {
        self.start_index = index;
        self.end_index = index;
    }
    fn get(&self) -> Self {
        Self {
            start_index: self.start_index,
            end_index: self.end_index,
        }
    }
    fn is_zero_dimensional(&self) -> bool {
        if self.len() == 1 { true } else { false }
    }
    fn len(&self) -> usize {
        self.end_index - self.start_index + 1 // проверка того что например end index меньше уже на стророне валидатора
    }
}

pub fn slice_borrow_transform<'a>(slices: &'a [Slice]) -> Vec<&'a Slice> {
    let mut res: Vec<&Slice> = Vec::new();
    let mut iterator = slices.iter().map(|x| res.push(x));
    loop {
        if !iterator.next().is_some() {
            break;
        }
    }
    res
}

pub fn get_var_slice(slice: Slice) -> Option<Vec<Var>> {
    let pack_vars: Result<&Arc<Mutex<Vec<Var>>>, ()> = get_or_init_vars();
    let unwrap_vars: &Arc<Mutex<Vec<Var>>> = unsafe { pack_vars.unwrap_unchecked() };
    let guard: std::sync::MutexGuard<'_, Vec<Var>> =
        unsafe { unwrap_vars.lock().unwrap_unchecked() };
    let slice_give: Option<&[Var]> = guard.get(slice.start_index..=slice.end_index);
    if slice_give.is_none() {
        None
    } else {
        Some(unsafe { slice_give.unwrap_unchecked().to_vec() })
    }
}

#[inline(never)]
#[cold]
//не юзай в цикле syka
pub fn take_var_slice(slice: Slice) -> bool {
    if !take_idxes_valid(&slice) {
        return false;
    }
    let pack_vars: Result<&Arc<Mutex<Vec<Var>>>, ()> = get_or_init_vars();
    let unwrap_vars: &Arc<Mutex<Vec<Var>>> = unsafe { pack_vars.unwrap_unchecked() };
    let mut guard: std::sync::MutexGuard<'_, Vec<Var>> =
        unsafe { unwrap_vars.lock().unwrap_unchecked() };

    let taked_vec_var_iter = guard
        .iter()
        .enumerate()
        .filter(|x: &(usize, _)| !(slice.start_index <= x.0 && x.0 <= slice.end_index));

    let vec: Vec<Var> = taked_vec_var_iter.map(|x: (_, &Var)| x.1.clone()).collect();
    *guard = vec;
    true
}

pub static TINY_VEC_SIZE: usize = 128;

#[derive(PartialEq)]
pub enum SliceErr {
    Overflow,
    Collision,
}

#[doc = "проверка пересечений slice (только для мальньких массивов)"]
pub fn check_slice_collision(slices: &[&Slice]) -> crate::optional_error::OptionErr<SliceErr> {
    let mut tiny_vec: ArrayVec<[usize; TINY_VEC_SIZE * 2]> = ArrayVec::default(); //2кб
    if slices.len() >= TINY_VEC_SIZE {
        return crate::optional_error::OptionErr::Err(SliceErr::Overflow);
    }
    for &item in slices {
        tiny_vec.push(item.start_index);
        tiny_vec.push(item.end_index);
    }
    for item1 in tiny_vec {
        for item2 in tiny_vec {
            if item1 == item2 {
                return crate::optional_error::OptionErr::Err(SliceErr::Collision);
            }
        }
    }

    crate::optional_error::OptionErr::None
}

pub fn take_var_slices(slices: &[&Slice]) -> bool {
    if !take_index_valid_slice_result_ret(slices) {
        #[cfg(debug_assertions)]
        {
            use crate::debug::display_utils::FormaterSliceFmt;

            debug_println!(
                "take_var_slices !take_index_valid_slice_result_ret vec res: \n{}",
                display_vec(
                    &take_index_valid_slice_all_ret(slices),
                    &FormaterSliceFmt::default()
                )
            )
        }
        return false;
    }
    let err_res: crate::optional_error::OptionErr<SliceErr> = check_slice_collision(slices);
    if err_res != crate::optional_error::OptionErr::None {
        let err_msg: String = match err_res {
            //да нет blyat ebанoго None сука факинг раст
            crate::optional_error::OptionErr::Err(SliceErr::Collision) => "Collision".to_string(),
            crate::optional_error::OptionErr::Err(SliceErr::Overflow) => "Overflow".to_string(),
            _ => "".to_string(),
        };
        debug_println!(
            "take_var_slices !check_slice_collision check_slice_collision: \n{}",
            err_msg
        );
    }
    let pack_vars: Result<&Arc<Mutex<Vec<Var>>>, ()> = get_or_init_vars();
    let unwrap_vars: &Arc<Mutex<Vec<Var>>> = unsafe { pack_vars.unwrap_unchecked() };
    let mut guard: std::sync::MutexGuard<'_, Vec<Var>> =
        unsafe { unwrap_vars.lock().unwrap_unchecked() };
    let mut vec: Vec<Var> = Vec::new();
    vec.reserve(slices.len() * 2);
    let guard_len: usize = guard.len();
    let slices_len: usize = slices.len();
    if guard_len < slices_len {
        debug_eprintln_fileinfo!(
            "take_var_slices g.uard.len = {}\tslices.len = {}",
            guard_len,
            slices_len
        );
        return false;
    }
    if slices_len * 4 > guard_len * 3 {
        guard.reserve(guard_len * 2 + 10);
    }

    for item in slices.iter() {
        unsafe {
            let slice: &Slice = *item;
            let mut iter_slice: usize = slice.start_index;
            let mut give: &Var;
            while iter_slice <= slice.end_index {
                //c like
                give = guard.get_unchecked(iter_slice);
                vec.push(give.clone());
                iter_slice += 1;
            }
        }
    }
    *guard = vec;
    true
}

pub fn take_var(index: usize) -> bool {
    let mut zero_dimenstity_slice: Slice = Slice::default();
    zero_dimenstity_slice.set_zero_dimensional(index);
    if !take_idxes_valid(&zero_dimenstity_slice) {
        return false;
    }
    let pack_vars: Result<&Arc<Mutex<Vec<Var>>>, ()> = get_or_init_vars();
    let unwrap_vars: &Arc<Mutex<Vec<Var>>> = unsafe { pack_vars.unwrap_unchecked() };
    let mut guard: std::sync::MutexGuard<'_, Vec<Var>> =
        unsafe { unwrap_vars.lock().unwrap_unchecked() };
    let taked_vec_var_iter = guard
        .iter()
        .enumerate()
        .filter(|x: &(usize, _)| x.0 != index);

    let vec: Vec<Var> = taked_vec_var_iter.map(|x: (_, &Var)| x.1.clone()).collect();
    *guard = vec;
    true
}

fn take_idxes_valid(slice: &Slice) -> bool {
    let pack_vars: Result<&Arc<Mutex<Vec<Var>>>, ()> = get_or_init_vars();
    let unwrap_vars: &Arc<Mutex<Vec<Var>>> = unsafe { pack_vars.unwrap_unchecked() };
    let guard: std::sync::MutexGuard<'_, Vec<Var>> =
        unsafe { unwrap_vars.lock().unwrap_unchecked() };
    let vars_len: usize = guard.len();
    if !(slice.start_index < vars_len || slice.start_index > 0) {
        debug_eprintln_fileinfo!(
            "take_idxes_valid false: start_idx = {}  vars_len = {}",
            slice.start_index,
            vars_len
        );
        return false;
    }
    if !(slice.start_index >= slice.end_index || slice.end_index < vars_len) {
        debug_eprintln_fileinfo!(
            "take_idxes_valid false: end_idx = {}  start_idx = {}  vars_len = {}",
            slice.end_index,
            slice.start_index,
            vars_len
        );
        return false;
    }
    true
}

#[inline(always)]
fn take_index_valid_slice_all_ret(slices: &[&Slice]) -> Vec<bool> {
    //бля я щас заметил что я Box вообще не помню чтоб юзал
    let len: usize = slices.len();
    let mut res_vec: Vec<bool> = Vec::new();
    res_vec.reserve(len * 2); //потому что вроде если больше 75% займет а он займет если обычный len будет увилечение в два
    for &item in slices {
        res_vec.push(take_idxes_valid(item));
    }
    res_vec
}

#[inline(always)]
fn internal_result_res(vec_all_ret: Vec<bool>) -> bool {
    for item in vec_all_ret {
        if !item {
            return false;
        }
    }
    true
}

#[inline(always)]
fn take_index_valid_slice_result_ret(slices: &[&Slice]) -> bool {
    let all_ret: Vec<bool> = take_index_valid_slice_all_ret(slices);
    internal_result_res(all_ret)
}

// Copyright 2019 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Parameterized string expansion

use self::Param::*;
use self::States::*;

use std::iter::repeat;

#[derive(Clone, Copy, PartialEq)]
enum States {
    Nothing,
    Delay,
    Percent,
    SetVar,
    GetVar,
    PushParam,
    CharConstant,
    CharClose,
    IntConstant(i32),
    FormatPattern(Flags, FormatState),
    SeekIfElse(usize),
    SeekIfElsePercent(usize),
    SeekIfEnd(usize),
    SeekIfEndPercent(usize),
}

#[derive(Copy, PartialEq, Clone)]
enum FormatState {
    Flags,
    Width,
    Precision,
}

/// Types of parameters a capability can use
#[allow(missing_docs)]
#[derive(Clone)]
pub enum Param {
    Number(i32),
    Words(String),
}

impl Default for Param {
    fn default() -> Self {
        Param::Number(0)
    }
}

/// An error from interpreting a parameterized string.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// Data was requested from the stack, but the stack didn't have enough elements.
    StackUnderflow,
    /// The type of the element(s) on top of the stack did not match the type that the operator
    /// wanted.
    TypeMismatch,
    /// An unrecognized format option was used.
    UnrecognizedFormatOption(char),
    /// An invalid variable name was used.
    InvalidVariableName(char),
    /// An invalid parameter index was used.
    InvalidParameterIndex(char),
    /// A malformed character constant was used.
    MalformedCharacterConstant,
    /// An integer constant was too large (overflowed an i32)
    IntegerConstantOverflow,
    /// A malformed integer constant was used.
    MalformedIntegerConstant,
    /// A format width constant was too large (overflowed a usize)
    FormatWidthOverflow,
    /// A format precision constant was too large (overflowed a usize)
    FormatPrecisionOverflow,
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        use self::Error::*;
        match self {
            StackUnderflow => f.write_str("not enough elements on the stack"),
            TypeMismatch => f.write_str("type mismatch"),
            UnrecognizedFormatOption(_) => f.write_str("unrecognized format option"),
            InvalidVariableName(_) => f.write_str("invalid variable name"),
            InvalidParameterIndex(_) => f.write_str("invalid parameter index"),
            MalformedCharacterConstant => f.write_str("malformed character constant"),
            IntegerConstantOverflow => f.write_str("integer constant computation overflowed"),
            MalformedIntegerConstant => f.write_str("malformed integer constant"),
            FormatWidthOverflow => f.write_str("format width constant computation overflowed"),
            FormatPrecisionOverflow => {
                f.write_str("format precision constant computation overflowed")
            }
        }
    }
}

impl ::std::error::Error for Error {}

/// Container for static and dynamic variable arrays
#[derive(Default)]
pub struct Variables {
    /// Static variables A-Z
    sta_vars: [Param; 26],
    /// Dynamic variables a-z
    dyn_vars: [Param; 26],
}

impl Variables {
    /// Return a new zero-initialized Variables
    pub fn new() -> Variables {
        Default::default()
    }
}

/// Expand a parameterized capability
///
/// # Arguments
/// * `cap`    - string to expand
/// * `params` - vector of params for %p1 etc
/// * `vars`   - Variables struct for %Pa etc
///
/// To be compatible with ncurses, `vars` should be the same between calls to `expand` for
/// multiple capabilities for the same terminal.
pub fn expand(cap: &[u8], params: &[Param], vars: &mut Variables) -> Result<Vec<u8>, Error> {
    let mut state = Nothing;

    // expanded cap will only rarely be larger than the cap itself
    let mut output = Vec::with_capacity(cap.len());

    let mut stack: Vec<Param> = Vec::new();

    // Copy parameters into a local vector for mutability
    let mut mparams = [
        Number(0),
        Number(0),
        Number(0),
        Number(0),
        Number(0),
        Number(0),
        Number(0),
        Number(0),
        Number(0),
    ];
    for (dst, src) in mparams.iter_mut().zip(params.iter()) {
        *dst = (*src).clone();
    }

    for &c in cap.iter() {
        let cur = c as char;
        let mut old_state = state;
        match state {
            Nothing => {
                if cur == '%' {
                    state = Percent;
                } else if cur == '$' {
                    state = Delay;
                } else {
                    output.push(c);
                }
            }
            Delay => {
                old_state = Nothing;
                if cur == '>' {
                    state = Nothing;
                }
            }
            Percent => {
                match cur {
                    '%' => {
                        output.push(c);
                        state = Nothing
                    }
                    'c' => {
                        match stack.pop() {
                            // if c is 0, use 0200 (128) for ncurses compatibility
                            Some(Number(0)) => output.push(128u8),
                            // Don't check bounds. ncurses just casts and truncates.
                            Some(Number(c)) => output.push(c as u8),
                            Some(_) => return Err(Error::TypeMismatch),
                            None => return Err(Error::StackUnderflow),
                        }
                    }
                    'p' => state = PushParam,
                    'P' => state = SetVar,
                    'g' => state = GetVar,
                    '\'' => state = CharConstant,
                    '{' => state = IntConstant(0),
                    'l' => match stack.pop() {
                        Some(Words(s)) => stack.push(Number(s.len() as i32)),
                        Some(_) => return Err(Error::TypeMismatch),
                        None => return Err(Error::StackUnderflow),
                    },
                    '+' | '-' | '/' | '*' | '^' | '&' | '|' | 'm' => {
                        match (stack.pop(), stack.pop()) {
                            (Some(Number(y)), Some(Number(x))) => stack.push(Number(match cur {
                                '+' => x + y,
                                '-' => x - y,
                                '*' => x * y,
                                '/' => x / y,
                                '|' => x | y,
                                '&' => x & y,
                                '^' => x ^ y,
                                'm' => x % y,
                                _ => unreachable!("logic error"),
                            })),
                            (Some(_), Some(_)) => return Err(Error::TypeMismatch),
                            _ => return Err(Error::StackUnderflow),
                        }
                    }
                    '=' | '>' | '<' | 'A' | 'O' => match (stack.pop(), stack.pop()) {
                        (Some(Number(y)), Some(Number(x))) => stack.push(Number(
                            if match cur {
                                '=' => x == y,
                                '<' => x < y,
                                '>' => x > y,
                                'A' => x > 0 && y > 0,
                                'O' => x > 0 || y > 0,
                                _ => unreachable!("logic error"),
                            } {
                                1
                            } else {
                                0
                            },
                        )),
                        (Some(_), Some(_)) => return Err(Error::TypeMismatch),
                        _ => return Err(Error::StackUnderflow),
                    },
                    '!' | '~' => match stack.pop() {
                        Some(Number(x)) => stack.push(Number(match cur {
                            '!' if x > 0 => 0,
                            '!' => 1,
                            '~' => !x,
                            _ => unreachable!("logic error"),
                        })),
                        Some(_) => return Err(Error::TypeMismatch),
                        None => return Err(Error::StackUnderflow),
                    },
                    'i' => match (&mparams[0], &mparams[1]) {
                        (&Number(x), &Number(y)) => {
                            mparams[0] = Number(x + 1);
                            mparams[1] = Number(y + 1);
                        }
                        (_, _) => return Err(Error::TypeMismatch),
                    },

                    // printf-style support for %doxXs
                    'd' | 'o' | 'x' | 'X' | 's' => {
                        if let Some(arg) = stack.pop() {
                            let flags = Flags::default();
                            let res = format(arg, FormatOp::from_char(cur), flags)?;
                            output.extend(res);
                        } else {
                            return Err(Error::StackUnderflow);
                        }
                    }
                    ':' | '#' | ' ' | '.' | '0'..='9' => {
                        let mut flags = Flags::default();
                        let mut fstate = FormatState::Flags;
                        match cur {
                            ':' => (),
                            '#' => flags.alternate = true,
                            ' ' => flags.space = true,
                            '.' => fstate = FormatState::Precision,
                            '0'..='9' => {
                                flags.width = cur as usize - '0' as usize;
                                fstate = FormatState::Width;
                            }
                            _ => unreachable!("logic error"),
                        }
                        state = FormatPattern(flags, fstate);
                    }

                    // conditionals
                    '?' | ';' => (),
                    't' => match stack.pop() {
                        Some(Number(0)) => state = SeekIfElse(0),
                        Some(Number(_)) => (),
                        Some(_) => return Err(Error::TypeMismatch),
                        None => return Err(Error::StackUnderflow),
                    },
                    'e' => state = SeekIfEnd(0),
                    c => return Err(Error::UnrecognizedFormatOption(c)),
                }
            }
            PushParam => {
                // params are 1-indexed
                stack.push(
                    mparams[match cur.to_digit(10) {
                        Some(d) => d as usize - 1,
                        None => return Err(Error::InvalidParameterIndex(cur)),
                    }]
                    .clone(),
                );
            }
            SetVar => match cur {
                'A'..='Z' => {
                    if let Some(arg) = stack.pop() {
                        let idx = (cur as u8) - b'A';
                        vars.sta_vars[idx as usize] = arg;
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
                'a'..='z' => {
                    if let Some(arg) = stack.pop() {
                        let idx = (cur as u8) - b'a';
                        vars.dyn_vars[idx as usize] = arg;
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
                _ => {
                    return Err(Error::InvalidVariableName(cur));
                }
            },
            GetVar => match cur {
                'A'..='Z' => {
                    let idx = (cur as u8) - b'A';
                    stack.push(vars.sta_vars[idx as usize].clone());
                }
                'a'..='z' => {
                    let idx = (cur as u8) - b'a';
                    stack.push(vars.dyn_vars[idx as usize].clone());
                }
                _ => {
                    return Err(Error::InvalidVariableName(cur));
                }
            },
            CharConstant => {
                stack.push(Number(i32::from(c)));
                state = CharClose;
            }
            CharClose => {
                if cur != '\'' {
                    return Err(Error::MalformedCharacterConstant);
                }
            }
            IntConstant(i) => {
                if cur == '}' {
                    stack.push(Number(i));
                    state = Nothing;
                } else if let Some(digit) = cur.to_digit(10) {
                    match i
                        .checked_mul(10)
                        .and_then(|i_ten| i_ten.checked_add(digit as i32))
                    {
                        Some(i) => {
                            state = IntConstant(i);
                            old_state = Nothing;
                        }
                        None => return Err(Error::IntegerConstantOverflow),
                    }
                } else {
                    return Err(Error::MalformedIntegerConstant);
                }
            }
            FormatPattern(ref mut flags, ref mut fstate) => {
                old_state = Nothing;
                match (*fstate, cur) {
                    (_, 'd') | (_, 'o') | (_, 'x') | (_, 'X') | (_, 's') => {
                        if let Some(arg) = stack.pop() {
                            let res = format(arg, FormatOp::from_char(cur), *flags)?;
                            output.extend(res);
                            // will cause state to go to Nothing
                            old_state = FormatPattern(*flags, *fstate);
                        } else {
                            return Err(Error::StackUnderflow);
                        }
                    }
                    (FormatState::Flags, '#') => {
                        flags.alternate = true;
                    }
                    (FormatState::Flags, '-') => {
                        flags.left = true;
                    }
                    (FormatState::Flags, '+') => {
                        flags.sign = true;
                    }
                    (FormatState::Flags, ' ') => {
                        flags.space = true;
                    }
                    (FormatState::Flags, '0'..='9') => {
                        flags.width = cur as usize - '0' as usize;
                        *fstate = FormatState::Width;
                    }
                    (FormatState::Width, '0'..='9') => {
                        flags.width = match flags
                            .width
                            .checked_mul(10)
                            .and_then(|w| w.checked_add(cur as usize - '0' as usize))
                        {
                            Some(width) => width,
                            None => return Err(Error::FormatWidthOverflow),
                        }
                    }
                    (FormatState::Width, '.') | (FormatState::Flags, '.') => {
                        *fstate = FormatState::Precision;
                    }
                    (FormatState::Precision, '0'..='9') => {
                        flags.precision = match flags
                            .precision
                            .checked_mul(10)
                            .and_then(|w| w.checked_add(cur as usize - '0' as usize))
                        {
                            Some(precision) => precision,
                            None => return Err(Error::FormatPrecisionOverflow),
                        }
                    }
                    _ => return Err(Error::UnrecognizedFormatOption(cur)),
                }
            }
            SeekIfElse(level) => {
                if cur == '%' {
                    state = SeekIfElsePercent(level);
                }
                old_state = Nothing;
            }
            SeekIfElsePercent(level) => {
                if cur == ';' {
                    if level == 0 {
                        state = Nothing;
                    } else {
                        state = SeekIfElse(level - 1);
                    }
                } else if cur == 'e' && level == 0 {
                    state = Nothing;
                } else if cur == '?' {
                    state = SeekIfElse(level + 1);
                } else {
                    state = SeekIfElse(level);
                }
            }
            SeekIfEnd(level) => {
                if cur == '%' {
                    state = SeekIfEndPercent(level);
                }
                old_state = Nothing;
            }
            SeekIfEndPercent(level) => {
                if cur == ';' {
                    if level == 0 {
                        state = Nothing;
                    } else {
                        state = SeekIfEnd(level - 1);
                    }
                } else if cur == '?' {
                    state = SeekIfEnd(level + 1);
                } else {
                    state = SeekIfEnd(level);
                }
            }
        }
        if state == old_state {
            state = Nothing;
        }
    }
    Ok(output)
}

#[derive(Copy, PartialEq, Clone, Default)]
struct Flags {
    width: usize,
    precision: usize,
    alternate: bool,
    left: bool,
    sign: bool,
    space: bool,
}

#[derive(Copy, Clone)]
enum FormatOp {
    Digit,
    Octal,
    Hex,
    #[allow(clippy::upper_case_acronyms)]
    HEX,
    String,
}

impl FormatOp {
    fn from_char(c: char) -> FormatOp {
        use self::FormatOp::*;
        match c {
            'd' => Digit,
            'o' => Octal,
            'x' => Hex,
            'X' => HEX,
            's' => String,
            _ => panic!("bad FormatOp char"),
        }
    }
}

fn format(val: Param, op: FormatOp, flags: Flags) -> Result<Vec<u8>, Error> {
    use self::FormatOp::*;
    let mut s = match val {
        Number(d) => {
            match op {
                Digit => {
                    // C doesn't take sign into account in precision calculation.
                    if flags.sign {
                        format!("{:+01$}", d, flags.precision + 1)
                    } else if d < 0 {
                        format!("{:01$}", d, flags.precision + 1)
                    } else if flags.space {
                        format!(" {:01$}", d, flags.precision)
                    } else {
                        format!("{:01$}", d, flags.precision)
                    }
                }
                Octal => {
                    if flags.alternate {
                        // Leading octal zero counts against precision.
                        format!("0{:01$o}", d, flags.precision.saturating_sub(1))
                    } else {
                        format!("{:01$o}", d, flags.precision)
                    }
                }
                Hex => {
                    if flags.alternate && d != 0 {
                        format!("0x{:01$x}", d, flags.precision)
                    } else {
                        format!("{:01$x}", d, flags.precision)
                    }
                }
                HEX => {
                    if flags.alternate && d != 0 {
                        format!("0X{:01$X}", d, flags.precision)
                    } else {
                        format!("{:01$X}", d, flags.precision)
                    }
                }
                String => return Err(Error::TypeMismatch),
            }
            .into_bytes()
        }
        Words(s) => match op {
            String => {
                let mut s = s.into_bytes();
                if flags.precision > 0 && flags.precision < s.len() {
                    s.truncate(flags.precision);
                }
                s
            }
            _ => return Err(Error::TypeMismatch),
        },
    };
    if flags.width > s.len() {
        let n = flags.width - s.len();
        if flags.left {
            s.extend(repeat(b' ').take(n));
        } else {
            let mut s_ = Vec::with_capacity(flags.width);
            s_.extend(repeat(b' ').take(n));
            s_.extend(s);
            s = s_;
        }
    }
    Ok(s)
}

#[cfg(test)]
mod test {
    use super::Param::{self, Number, Words};
    use super::{expand, Variables};
    use std::result::Result::Ok;

    #[test]
    fn test_basic_setabf() {
        let s = b"\\E[48;5;%p1%dm";
        assert_eq!(
            expand(s, &[Number(1)], &mut Variables::new()).unwrap(),
            "\\E[48;5;1m".bytes().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_multiple_int_constants() {
        assert_eq!(
            expand(b"%{1}%{2}%d%d", &[], &mut Variables::new()).unwrap(),
            "21".bytes().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_op_i() {
        let mut vars = Variables::new();
        assert_eq!(
            expand(
                b"%p1%d%p2%d%p3%d%i%p1%d%p2%d%p3%d",
                &[Number(1), Number(2), Number(3)],
                &mut vars
            ),
            Ok("123233".bytes().collect::<Vec<_>>())
        );
        assert_eq!(
            expand(b"%p1%d%p2%d%i%p1%d%p2%d", &[], &mut vars),
            Ok("0011".bytes().collect::<Vec<_>>())
        );
    }

    #[test]
    fn test_param_stack_failure_conditions() {
        let mut varstruct = Variables::new();
        let vars = &mut varstruct;
        fn get_res(
            fmt: &str,
            cap: &str,
            params: &[Param],
            vars: &mut Variables,
        ) -> Result<Vec<u8>, super::Error> {
            let mut u8v: Vec<_> = fmt.bytes().collect();
            u8v.extend(cap.as_bytes().iter().cloned());
            expand(&u8v, params, vars)
        }

        let caps = ["%d", "%c", "%s", "%Pa", "%l", "%!", "%~"];
        for &cap in &caps {
            let res = get_res("", cap, &[], vars);
            assert!(
                res.is_err(),
                "Op {} succeeded incorrectly with 0 stack entries",
                cap
            );
            let p = if cap == "%s" || cap == "%l" {
                Words("foo".to_owned())
            } else {
                Number(97)
            };
            let res = get_res("%p1", cap, &[p], vars);
            assert!(
                res.is_ok(),
                "Op {} failed with 1 stack entry: {}",
                cap,
                res.err().unwrap()
            );
        }
        let caps = ["%+", "%-", "%*", "%/", "%m", "%&", "%|", "%A", "%O"];
        for &cap in &caps {
            let res = expand(cap.as_bytes(), &[], vars);
            assert!(
                res.is_err(),
                "Binop {} succeeded incorrectly with 0 stack entries",
                cap
            );
            let res = get_res("%{1}", cap, &[], vars);
            assert!(
                res.is_err(),
                "Binop {} succeeded incorrectly with 1 stack entry",
                cap
            );
            let res = get_res("%{1}%{2}", cap, &[], vars);
            assert!(
                res.is_ok(),
                "Binop {} failed with 2 stack entries: {}",
                cap,
                res.err().unwrap()
            );
        }
    }

    #[test]
    fn test_push_bad_param() {
        assert!(expand(b"%pa", &[], &mut Variables::new()).is_err());
    }

    #[test]
    fn test_comparison_ops() {
        let v = [
            ('<', [1u8, 0u8, 0u8]),
            ('=', [0u8, 1u8, 0u8]),
            ('>', [0u8, 0u8, 1u8]),
        ];
        for &(op, bs) in &v {
            let s = format!("%{{1}}%{{2}}%{}%d", op);
            let res = expand(s.as_bytes(), &[], &mut Variables::new());
            assert!(res.is_ok(), "{}", res.err().unwrap());
            assert_eq!(res.unwrap(), vec![b'0' + bs[0]]);
            let s = format!("%{{1}}%{{1}}%{}%d", op);
            let res = expand(s.as_bytes(), &[], &mut Variables::new());
            assert!(res.is_ok(), "{}", res.err().unwrap());
            assert_eq!(res.unwrap(), vec![b'0' + bs[1]]);
            let s = format!("%{{2}}%{{1}}%{}%d", op);
            let res = expand(s.as_bytes(), &[], &mut Variables::new());
            assert!(res.is_ok(), "{}", res.err().unwrap());
            assert_eq!(res.unwrap(), vec![b'0' + bs[2]]);
        }
    }

    #[test]
    fn test_conditionals() {
        let mut vars = Variables::new();
        let s = b"\\E[%?%p1%{8}%<%t3%p1%d%e%p1%{16}%<%t9%p1%{8}%-%d%e38;5;%p1%d%;m";
        let res = expand(s, &[Number(1)], &mut vars);
        assert!(res.is_ok(), "{}", res.err().unwrap());
        assert_eq!(res.unwrap(), "\\E[31m".bytes().collect::<Vec<_>>());
        let res = expand(s, &[Number(8)], &mut vars);
        assert!(res.is_ok(), "{}", res.err().unwrap());
        assert_eq!(res.unwrap(), "\\E[90m".bytes().collect::<Vec<_>>());
        let res = expand(s, &[Number(42)], &mut vars);
        assert!(res.is_ok(), "{}", res.err().unwrap());
        assert_eq!(res.unwrap(), "\\E[38;5;42m".bytes().collect::<Vec<_>>());
    }

    #[test]
    fn test_format() {
        let mut varstruct = Variables::new();
        let vars = &mut varstruct;
        assert_eq!(
            expand(
                b"%p1%s%p2%2s%p3%2s%p4%.2s",
                &[
                    Words("foo".to_owned()),
                    Words("foo".to_owned()),
                    Words("f".to_owned()),
                    Words("foo".to_owned())
                ],
                vars
            ),
            Ok("foofoo ffo".bytes().collect::<Vec<_>>())
        );
        assert_eq!(
            expand(b"%p1%:-4.2s", &[Words("foo".to_owned())], vars),
            Ok("fo  ".bytes().collect::<Vec<_>>())
        );

        assert_eq!(
            expand(b"%p1%d%p1%.3d%p1%5d%p1%:+d", &[Number(1)], vars),
            Ok("1001    1+1".bytes().collect::<Vec<_>>())
        );
        assert_eq!(
            expand(
                b"%p1%o%p1%#o%p2%6.4x%p2%#6.4X",
                &[Number(15), Number(27)],
                vars
            ),
            Ok("17017  001b0X001B".bytes().collect::<Vec<_>>())
        );
        assert_eq!(
            expand(
                b"%p1%.5d%p1% .5d%p1%:+.5d%p2%.5d%p2% .5d%p2%:+.5d",
                &[Number(15), Number(-15)],
                vars
            ),
            Ok("00015 00015+00015-00015-00015-00015"
                .bytes()
                .collect::<Vec<_>>())
        );
    }
}