use more_debug_asserts::inner::*;
use obsidian_sync_lib::debug_eprintln;
use core::fmt;

#[cold]
#[track_caller]
#[inline(never)]
pub fn not_panic_assert_failed_impl(
    left: &dyn fmt::Debug,
    right: &dyn fmt::Debug,
    ty: AssertType,
    msg: Option<fmt::Arguments<'_>>,
){
    let compare: &str = match ty {
        AssertType::Lt => "<",
        AssertType::Gt => ">",
        AssertType::Le => "<=",
        AssertType::Ge => ">=",
    };
    if let Some(msg) = msg {
        debug_eprintln!(
            "assertion failed: `(left {} right)`\n  left: `{:?}`,\n right: `{:?}`: {}",
            compare, left, right, msg,
        );
    } else {
        debug_eprintln!(
            "assertion failed: `(left {} right)`\n  left: `{:?}`,\n right: `{:?}`: {}",
            compare, left, right, "",
        );
    }
}

