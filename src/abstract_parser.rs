use crate::{abstract__tokinezer::*, utils::TimePoint};

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
    ) -> Option<(crate::number_utils::DivisionError, String)> {
        //fix Optinal Err
        let cmp_time: Result<TimePoint, (crate::number_utils::DivisionError, String)> =
            TimePoint::new(0, 0, 1, 0, 0);
        let mut cmp_time_unwrap: TimePoint = TimePoint::default(); //todo пофиксить Error E0381
        if let Ok(val) = cmp_time {
            cmp_time_unwrap = val;
        } else if let Err(err) = cmp_time {
            return Some(err);
        }
        if time <= cmp_time_unwrap {}
        *self = Self::Time(time);
        None
    }
}

struct Var {}

//pub static VARS = OnceLock<Arc<Mutex<TokenStruct>>> = OnceLock::new();
