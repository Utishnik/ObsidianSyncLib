use crate::debug::debug_and_test_utils::Colors;
use crate::debug::debug_and_test_utils::reset_color_eprint;
use crate::debug::debug_and_test_utils::set_color_eprint;
use crate::debug_eprintln_fileinfo;
use std::any::Any;
use std::any::TypeId;

#[doc = "type_id это id value"]
pub struct TypedResult {
    value: Box<dyn Any>,
    type_id: TypeId,
    type_name: String,
}

pub enum TypedValue {
    I32(i32),
    I64(i64),
    F64(f64),
    Usize(usize),
    String(String),
    Bool(bool),
    // другие типы...
}

macro_rules! as_types_val {
    ($name_fn:ident,$enum_type:path,$ret_type:ty) => {
        pub fn $name_fn(&self) -> Option<$ret_type> {
            match self {
                $enum_type(v) => Some(*v),
                _ => None,
            }
        }
    };
}

impl TypedValue {
    as_types_val!(as_i32, TypedValue::I32, i32);
    as_types_val!(as_i64, TypedValue::I64, i64);
    as_types_val!(as_usize, TypedValue::Usize, usize);
    as_types_val!(as_f64, TypedValue::F64, f64);
    as_types_val!(as_bool, TypedValue::Bool, bool);

    pub fn as_string(&self) -> Option<String> {
        match self {
            TypedValue::String(v) => Some(v.clone()),
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

pub unsafe fn obsydian_downcast_ref_unchecked<'a, T: Any>(value: &'a (dyn Any + 'static)) -> &'a T {
    debug_assert!(value.is::<T>());
    // SAFETY: caller guarantees that T is the correct type
    unsafe { &*(value as *const dyn Any as *const T) }
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
            //когда станет стабильной TodoSome(self.value.downcast_ref_unchecked::<T>())
            unsafe { *obsydian_downcast_ref_unchecked(self.value.as_ref()) }
        } else {
            None
        }
    }

    fn downcast<T: 'static>(self) -> Result<T, Self> {
        if self.type_id == TypeId::of::<T>() {
            match self.value.downcast::<T>() {
                Ok(box_t) => Ok(*box_t),
                Err(value) => {
                    set_color_eprint(Colors::Red);
                    debug_eprintln_fileinfo!("fn downcast\ttype_name: {}", self.type_name);
                    reset_color_eprint();
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
