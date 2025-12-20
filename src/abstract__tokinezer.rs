use crate::splitt_b_space;
use crate::tokinezed;
use std::default;
use std::error::Error as OtherError;

#[derive(Clone, Debug)]
pub struct Pos {
    col: usize,
    line: usize,
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
    fn get_pos(&self);
    fn size(&self) -> usize;
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

pub fn smart_abstract_multiconstruction_value_parse<F, T, E>(
    fns: Vec<F>,
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
        return Err(unsafe{val.err.unwrap_unchecked()});
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
    for item in vals.to_vec().iter().enumerate(){
        let (i,v) = item;
        ret[i] = skip_value(v.clone());
    }
    ret
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
