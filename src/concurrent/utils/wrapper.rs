use anyhow::Result;
use std::time::Instant;

use crate::structs::{IntoFlag, Verbose};

#[inline]
pub fn result_unwrapper<O>(r: Result<O>) -> O
where
    O: Default,
{
    r.unwrap_or_else(|err| {
        eprintln!("{}", &err);
        O::default()
    })
}

#[inline]
pub async fn timing_decorator<F, R>(context: &str, f: F, v: Verbose) -> R
where
    F: futures::Future<Output = R>,
{
    if v.into_flag() {
        let start = Instant::now();
        let f: R = f.await;
        eprintln!("{}: {}", context, start.elapsed().as_secs_f64());
        f
    } else {
        f.await
    }
}

#[inline]
pub async fn error_unwrapper<F, R>(f: F) -> R
where
    F: futures::Future<Output = Result<R>>,
    R: Default,
{
    f.await.unwrap_or_else(|err| {
        eprintln!("{}", &err);
        R::default()
    })
}
