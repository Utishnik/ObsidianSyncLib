use crate::splitt_b_space;
use crate::tokinezed;
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
    pub fn conver_to_idx(&self, cfg: &str) -> Result<usize,()> {
        let mut idx: usize=0;
        let ret_res: Result<tokinezed::TokenStruct, ()> = splitt_b_space(cfg.to_string(), None, Some("\n".to_string()));
        if let Err(err) = ret_res{
            return Err(err);
        }
        else if let Ok(ok) = ret_res{
            //надо суммировать длину каждой строки в векторе вплоть до нужной
            for item in ok.tok_values.iter().enumerate(){
                let (i,str) = item;
                let str_len: usize = str.len();
                if i != self.line{
                    idx+=str_len;
                }
                else{//нужно до конкретно col
                    idx+=self.col;
                }
            }
        }
        Ok(idx)
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

pub fn smart_abstract_multiconstruction_value_parse<F, T, E>(
    fns: Vec<F>,
    idx: &mut usize,
) -> Vec<AbstractParseValue<T, E>>
where
    F: FnOnce(&mut usize) -> AbstractParseValue<T, E> + std::clone::Clone,
    T: AbstractValue + std::clone::Clone,
    E: std::clone::Clone,
{
    let mut result: Vec<AbstractParseValue<T, E>> = Vec::new();
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
        ret_res = Some((result.unwrap()).clone());
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
        return Err(val.err.unwrap());
    }
    Ok((val.pos, val.val.clone().unwrap())) //todo сделать конверт Pos в обычный index: usize
}
