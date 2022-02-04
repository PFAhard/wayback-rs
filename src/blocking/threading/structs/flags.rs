pub trait IntoFlag {
    fn into_flag(self) -> bool;

    fn select<I>(self, one: I, sec: I) -> I;

    fn restrict<I>(self, input: I)
    where
        I: FnOnce();
}

#[derive(Debug, Clone, Copy)]
pub enum SubsFlag {
    Enable,
    Disable,
}

impl From<bool> for SubsFlag {
    fn from(x: bool) -> Self {
        if x {
            Self::Enable
        } else {
            Self::Disable
        }
    }
}

impl From<SubsFlag> for bool {
    fn from(f: SubsFlag) -> bool {
        match f {
            SubsFlag::Enable => true,
            SubsFlag::Disable => false,
        }
    }
}

impl IntoFlag for SubsFlag {
    fn into_flag(self) -> bool {
        match self {
            SubsFlag::Enable => true,
            SubsFlag::Disable => false,
        }
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

#[derive(Debug, Clone, Copy)]
pub enum NetThreads {
    Enable,
    Disable,
}

impl From<bool> for NetThreads {
    fn from(x: bool) -> Self {
        if x {
            Self::Disable
        } else {
            Self::Enable
        }
    }
}

impl IntoFlag for NetThreads {
    fn into_flag(self) -> bool {
        match self {
            NetThreads::Enable => true,
            NetThreads::Disable => false,
        }
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

#[derive(Debug, Clone, Copy)]
pub enum Expensive {
    Enable,
    Disable,
}

impl From<bool> for Expensive {
    fn from(x: bool) -> Self {
        if x {
            Self::Enable
        } else {
            Self::Disable
        }
    }
}

impl IntoFlag for Expensive {
    fn into_flag(self) -> bool {
        match self {
            Expensive::Enable => true,
            Expensive::Disable => false,
        }
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
