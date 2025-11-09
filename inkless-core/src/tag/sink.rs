use core::ops::ControlFlow;

use crate::{grapheme::gph, renderable::RenderableError, tag::Tag};

pub trait TagSink<T: Tag> {
    type Result: From<RenderableError>;

    fn append(&mut self, grapheme: &gph, tag: T) -> ControlFlow<()>;
    fn gap(&mut self) -> ControlFlow<()>;
    fn finalize_line(&mut self) -> ControlFlow<()>;

    fn finalize(self) -> Self::Result;
}

impl<T, E: From<RenderableError>> From<RenderableError> for Result<T, E> {
    fn from(v: RenderableError) -> Self {
        Err(v.into())
    }
}
