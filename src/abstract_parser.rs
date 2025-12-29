use crate::debug_eprintln_fileinfo;
use crate::{abstract__tokinezer::*, debug_println, utils::TimePoint};
use std::marker::PhantomData;
use std::slice::SliceIndex;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;

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

pub fn push_var(var: Var) {
    let pack_vars: Result<&Arc<Mutex<Vec<Var>>>, ()> = get_or_init_vars();
    let unwrap_vars: &Arc<Mutex<Vec<Var>>> = unsafe { pack_vars.unwrap_unchecked() };
    let mut guard: std::sync::MutexGuard<'_, Vec<Var>> =
        unsafe { unwrap_vars.lock().unwrap_unchecked() }; //обработка отравления на стороне get_or_init_vars
    guard.push(var);
}

pub fn get_var(index: usize) -> Option<Var> {
    let pack_vars: Result<&Arc<Mutex<Vec<Var>>>, ()> = get_or_init_vars();
    let unwrap_vars: &Arc<Mutex<Vec<Var>>> = unsafe { pack_vars.unwrap_unchecked() };
    let guard: std::sync::MutexGuard<'_, Vec<Var>> =
        unsafe { unwrap_vars.lock().unwrap_unchecked() };
    guard.get(index).cloned()
}

pub struct Slice {
    start_index: usize,
    end_index: usize,
}

impl Slice {
    fn set(&mut self, start_idx: usize, end_idx: usize) {
        self.start_index = start_idx;
        self.end_index = end_idx;
    }
    fn get(&self) -> Self {
        Self {
            start_index: self.start_index,
            end_index: self.end_index,
        }
    }
    fn is_zero_dimensional(&self) -> bool {
        if self.start_index == self.end_index {
            true
        } else {
            false
        }
    }
}

pub fn get_var_slice(start_idx: usize, end_idx: usize) -> Option<Vec<Var>> {
    let pack_vars: Result<&Arc<Mutex<Vec<Var>>>, ()> = get_or_init_vars();
    let unwrap_vars: &Arc<Mutex<Vec<Var>>> = unsafe { pack_vars.unwrap_unchecked() };
    let guard: std::sync::MutexGuard<'_, Vec<Var>> =
        unsafe { unwrap_vars.lock().unwrap_unchecked() };
    let slice_give: Option<&[Var]> = guard.get(start_idx..=end_idx);
    if slice_give.is_none() {
        None
    } else {
        Some(unsafe { slice_give.unwrap_unchecked().to_vec() })
    }
}

//todo проверки не выхода
pub fn take_var_slice(start_idx: usize, end_idx: usize) -> bool {
    if !take_idxes_valid(start_idx, end_idx) {
        return false;
    }
    let pack_vars: Result<&Arc<Mutex<Vec<Var>>>, ()> = get_or_init_vars();
    let unwrap_vars: &Arc<Mutex<Vec<Var>>> = unsafe { pack_vars.unwrap_unchecked() };
    let mut guard: std::sync::MutexGuard<'_, Vec<Var>> =
        unsafe { unwrap_vars.lock().unwrap_unchecked() };

    let taked_vec_var_iter = guard
        .iter()
        .enumerate()
        .filter(|x: &(usize, _)| !(start_idx <= x.0 && x.0 <= end_idx));

    let vec: Vec<Var> = taked_vec_var_iter.map(|x: (_, &Var)| x.1.clone()).collect();
    *guard = vec;
    true
}

pub fn take_var(index: usize) -> bool {
    if !take_idxes_valid(index, index) {
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

fn take_idxes_valid(start_idx: usize, end_idx: usize) -> bool {
    let pack_vars: Result<&Arc<Mutex<Vec<Var>>>, ()> = get_or_init_vars();
    let unwrap_vars: &Arc<Mutex<Vec<Var>>> = unsafe { pack_vars.unwrap_unchecked() };
    let guard: std::sync::MutexGuard<'_, Vec<Var>> =
        unsafe { unwrap_vars.lock().unwrap_unchecked() };
    let vars_len: usize = guard.len();
    if !(start_idx < vars_len && start_idx > 0) {
        debug_eprintln_fileinfo!(
            "take_idxes_valid false: start_idx = {}  vars_len = {}",
            start_idx,
            vars_len
        );
        return false;
    }
    if !(end_idx >= start_idx && end_idx < vars_len) {
        debug_eprintln_fileinfo!(
            "take_idxes_valid false: end_idx = {}  start_idx = {}  vars_len = {}",
            end_idx,
            start_idx,
            vars_len
        );
        return false;
    }
    true
}

fn take_index_valid_slice() {}
