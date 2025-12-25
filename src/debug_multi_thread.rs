use crate::debug_eprintln_fileinfo;
use crate::debug_println;
use crate::debug_println_fileinfo;
use std::any::Any;
use std::any::TypeId;
use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::num::ParseIntError;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub struct TypedResult {
    value: Box<dyn Any>,
    type_id: TypeId,
    type_name: String,
}

pub enum TypedValue {
    I32(i32),
    I64(i64),
    F64(f64),
    USIZE(usize),
    String(String),
    Bool(bool),
    // другие типы...
}

impl TypedValue {
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            TypedValue::I32(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            TypedValue::I64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_usize(&self) -> Option<usize> {
        match self {
            TypedValue::USIZE(v) => Some(*v),
            _ => None,
        }
    }

    pub fn f64(&self) -> Option<f64> {
        match self {
            TypedValue::F64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            TypedValue::String(v) => Some(v.clone()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TypedValue::Bool(v) => Some(*v),
            _ => None,
        }
    }
}

impl TypedResult {
    pub fn get_typed_value(&self) -> Option<TypedValue> {
        match self.type_id {
            t if t == TypeId::of::<i32>() => {
                self.downcast_ref::<i32>().map(|v| TypedValue::I32(*v))
            }
            t if t == TypeId::of::<String>() => self
                .downcast_ref::<String>()
                .map(|v| TypedValue::String(v.clone())),
            t if t == TypeId::of::<bool>() => {
                self.downcast_ref::<bool>().map(|v| TypedValue::Bool(*v))
            }
            _ => None,
        }
    }
}

impl TypedResult {
    pub fn new<T: 'static + std::fmt::Debug>(value: T) -> Self {
        Self {
            value: Box::new(value),
            type_id: TypeId::of::<T>(),
            type_name: std::any::type_name::<T>().to_string(),
        }
    }

    pub fn get_value(&self) -> Result<Option<Box<dyn Any + '_>>, TypedResult> {
        let res: Result<_, TypedResult> = Ok(self.get_typed_value());
        match res {
            Ok(x) => Ok(Some(Box::new(x))),
            Err(e) => Err(e),
        }
    }

    fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        if self.type_id == TypeId::of::<T>() {
            self.value.downcast_ref::<T>()
        } else {
            None
        }
    }

    fn downcast<T: 'static>(self) -> Result<T, Self> {
        if self.type_id == TypeId::of::<T>() {
            match self.value.downcast::<T>() {
                Ok(box_t) => Ok(*box_t),
                Err(value) => {
                    debug_eprintln_fileinfo!("fn downcast\ttype_name: {}", self.type_name);
                    Err(Self {
                        value,
                        type_id: self.type_id,
                        type_name: self.type_name,
                    })
                }
            }
        } else {
            Err(self)
        }
    }

    fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        if self.type_id == TypeId::of::<T>() {
            self.value.downcast_mut::<T>()
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! call_functions {
    ($($func:expr => ($($arg:expr),*) -> $ret:ty),+) => {{
        use std::any::Any;
        use crate::debug_multi_thread::*;
        let mut results: Vec<debug_multi_thread::TypedResult> = Vec::new();

        $(
            let result: $ret = $func($($arg),*);
            let res: TypedResult = TypedResult::new(result);

            results.push(res);//get и set
        )+

        debug_println!("len result: {}",results.len());

        results
    }};

}
