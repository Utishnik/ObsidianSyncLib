#[allow(missing_docs)]
pub trait Zero {
    #[doc = "просто для каждого типа опредляет метод zero() которые возвращет ноль его типа"]
    fn zero() -> Self;
}

macro_rules! zero_decl {
    ($ret_type: ty) => {
    impl Zero for $ret_type{
        fn zero() -> Self {
            0
        }
    }
    };
}
macro_rules! float_zero_decl {
    ($ret_type: ty) => {
    impl Zero for $ret_type{
        fn zero() -> Self {
            0.0
        }
    }
    };
}
zero_decl!(u8);
zero_decl!(u16);
zero_decl!(u32);
zero_decl!(u64);
zero_decl!(u128);
zero_decl!(usize);
zero_decl!(i8);
zero_decl!(i16);
zero_decl!(i32);
zero_decl!(i64);
zero_decl!(i128);
//zero_decl!(f16); // unstable
float_zero_decl!(f32);
float_zero_decl!(f64);
//zero_decl!(f128); //unstable
