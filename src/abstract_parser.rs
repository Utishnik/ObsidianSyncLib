use crate::debug_eprintln_fileinfo;
use crate::display_vec;
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
            debug_println!(
                "take_var_slices !take_index_valid_slice_result_ret vec res: \n{}",
                display_vec(&take_index_valid_slice_all_ret(slices), " ,".to_string())
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
