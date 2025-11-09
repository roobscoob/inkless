use core::ops::ControlFlow;

use thiserror::Error;

use crate::{
    grapheme::gph,
    renderable::RenderableError,
    tag::{Tag, sink::TagSink},
    writer::character::CharacterWriter,
};

pub struct Plaintext<W: CharacterWriter> {
    writer: W,
    result: Result<(), W::Error>,
}

impl<W: CharacterWriter> From<W> for Plaintext<W> {
    fn from(value: W) -> Self {
        Plaintext {
            writer: value,
            result: Ok(()),
        }
    }
}

#[derive(Error, Debug)]
pub enum PlaintextError<E> {
    Renderable(#[from] RenderableError),
    Writer(E),
}

impl<W: CharacterWriter, T: Tag> TagSink<T> for Plaintext<W> {
    type Result = Result<W, PlaintextError<W::Error>>;

    fn append(&mut self, grapheme: &gph, _: T) -> core::ops::ControlFlow<()> {
        self.result = self.writer.write_str(grapheme.as_str());

        if self.result.is_err() {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }

    fn gap(&mut self) -> ControlFlow<()> {
        self.result = self.writer.write_str(" ");

        if self.result.is_err() {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }

    fn finalize_line(&mut self) -> ControlFlow<()> {
        self.result = self.writer.write_str("\n");

        if self.result.is_err() {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }

    fn finalize(self) -> Self::Result {
        self.result
            .map_err(PlaintextError::Writer)
            .map(|_| self.writer)
    }
}
