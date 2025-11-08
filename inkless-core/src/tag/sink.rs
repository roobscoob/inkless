use core::ops::ControlFlow;

use crate::{grapheme::gph, tag::Tag};

pub trait TagSink<T: Tag> {
    type Result;

    fn append_tagged(&mut self, grapheme: &gph, tag: T) -> ControlFlow<()>;
    fn append(&mut self, grapheme: &gph) -> ControlFlow<()>;
    fn finalize_line(&mut self) -> ControlFlow<()>;

    fn finalize(self) -> Self::Result;
}
