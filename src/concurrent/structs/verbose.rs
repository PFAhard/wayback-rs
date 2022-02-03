use crate::structs::IntoFlag;
use async_trait::async_trait;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn app_trace(v: Verbose) {
    if let Some(level) = v.into_tracing_level() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(level)
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Verbose {
    None,
    Timing,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<u64> for Verbose {
    fn from(v: u64) -> Self {
        match v {
            0 => Self::None,
            1 => Self::Timing,
            2 => Self::Warn,
            3 => Self::Info,
            4 => Self::Debug,
            _ => Self::Trace,
        }
    }
}

#[async_trait]
impl IntoFlag for Verbose {
    fn into_flag(self) -> bool {
        !matches!(self, Self::None)
    }

    fn select<I>(self, one: I, sec: I) -> I {
        if self.into_flag() {
            one
        } else {
            sec
        }
    }

    #[cfg(feature = "async")]
    async fn restrict<I>(self, input: I)
    where
        I: futures::Future<Output = ()> + Send
    {
        input.await;
    }
}

impl Verbose {
    fn into_tracing_level(self) -> Option<Level> {
        match self {
            Verbose::None => Option::None,
            Verbose::Timing => Some(Level::ERROR),
            Verbose::Warn => Some(Level::WARN),
            Verbose::Info => Some(Level::INFO),
            Verbose::Debug => Some(Level::DEBUG),
            Verbose::Trace => Some(Level::TRACE), 
        }
    }
}