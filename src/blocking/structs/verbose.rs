use crate::structs::IntoFlag;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub(crate) fn app_trace(v: Verbose) {
    if let Some(level) = v.into_tracing_level() {
        let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Verbose {
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

    fn restrict<I>(self, input: I)
    where
        I: FnOnce(),
    {
        if self.into_flag() {
            input();
        }
    }
}

impl Verbose {
    pub(crate) fn into_tracing_level(self) -> Option<Level> {
        match self {
            Verbose::None => Option::None,
            Verbose::Timing => Some(Level::DEBUG),
            Verbose::Warn | Verbose::Info | Verbose::Debug | Verbose::Trace => Some(Level::TRACE),
        }
    }
}
