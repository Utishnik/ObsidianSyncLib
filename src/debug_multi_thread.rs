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
use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;

struct TypedResult {
    value: Box<dyn Any>,
    type_id: TypeId,
    type_name: String,
}

impl TypedResult {
    fn new<T: 'static + std::fmt::Debug>(value: T) -> Self {
        Self {
            value: Box::new(value),
            type_id: TypeId::of::<T>(),
            type_name: std::any::type_name::<T>().to_string(),
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
                Err(value) => Err(Self {
                    value,
                    type_id: self.type_id,
                    type_name: self.type_name,
                }),
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
        let mut results: Vec<Box<dyn Any>> = Vec::new();

        $(
            let result: $ret = $func($($arg),*);
            results.push(Box::new(result));
        )+

        debug_println!("len result: {}",results.len());

        results
    }};

}
