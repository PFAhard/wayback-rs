use async_trait::async_trait;

#[async_trait]
pub trait IntoFlag {
    fn into_flag(self) -> bool;

    fn select<I>(self, one: I, sec: I) -> I;

    async fn restrict<I>(self, input: I)
    where
        I: futures::Future<Output = ()> + Send;
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

#[async_trait]
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

    async fn restrict<I>(self, input: I)
    where
        I: futures::Future<Output = ()> + Send
    {
        input.await;
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
            Self::Enable
        } else {
            Self::Disable
        }
    }
}

#[async_trait]
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

    async fn restrict<I>(self, input: I)
    where
        I: futures::Future<Output = ()> + Send
    {
        input.await;
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

#[async_trait]
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

    async fn restrict<I>(self, input: I)
    where
        I: futures::Future<Output = ()> + Send
    {
        input.await;
    }
}
