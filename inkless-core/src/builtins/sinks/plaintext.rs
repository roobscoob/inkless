use core::ops::ControlFlow;

use crate::{
    grapheme::gph,
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

impl<W: CharacterWriter, T: Tag> TagSink<T> for Plaintext<W> {
    type Result = Result<W, W::Error>;

    fn append(&mut self, grapheme: &gph) -> core::ops::ControlFlow<()> {
        self.result = self.writer.write_str(grapheme.as_str());

        if self.result.is_err() {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }

    fn append_tagged(&mut self, grapheme: &gph, _: T) -> ControlFlow<()> {
        self.result = self.writer.write_str(grapheme.as_str());

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
        self.result.map(|_| self.writer)
    }
}
