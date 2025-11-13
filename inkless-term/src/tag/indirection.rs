use core::ops::Deref;

use crate::tag::{AnsiTag, default::Ansi};

pub trait AnsiDeref {
    fn deref(&self) -> &dyn AnsiTag;
}

impl<T: Deref> AnsiDeref for T
where
    T::Target: AnsiTag + Sized,
{
    fn deref(&self) -> &dyn AnsiTag {
        &**self
    }
}

impl AnsiDeref for Ansi {
    fn deref(&self) -> &dyn AnsiTag {
        self
    }
}
