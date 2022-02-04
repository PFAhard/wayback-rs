use anyhow::Result;
use std::time::Instant;

use crate::blocking::structs::{IntoFlag, Verbose};

#[inline]
pub(crate) fn result_unwrapper<O>(r: Result<O>) -> O
where
    O: Default,
{
    r.unwrap_or_else(|err| {
        eprintln!("{}", &err);
        O::default()
    })
}

#[inline]
pub(crate) fn timing_decorator<F, R>(context: &str, f: F, v: Verbose) -> R
where
    F: FnOnce() -> R,
{
    if v.into_flag() {
        let start = Instant::now();
        let f: R = f();
        eprintln!("{}: {}", context, start.elapsed().as_secs_f64());
        f
    } else {
        f()
    }
}

#[inline]
pub(crate) fn error_unwrapper<F, R>(f: F) -> R
where
    F: FnOnce() -> Result<R>,
    R: Default,
{
    f().unwrap_or_else(|err| {
        eprintln!("{}", &err);
        R::default()
    })
}
