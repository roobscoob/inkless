use core::ops::{ControlFlow, Deref};

use inkless_core::{
    grapheme::gph,
    renderable::RenderableError,
    tag::{Tag, sink::TagSink},
    writer::character::CharacterWriter,
};
use thiserror::Error;

use crate::{
    delta::{
        write_background_color_delta, write_blink_delta, write_concealed_delta,
        write_foreground_color_delta, write_hyperlink_delta, write_intensity_delta,
        write_italic_delta, write_strikethrough_delta, write_underline_color_delta,
        write_underline_style_delta,
    },
    support::AnsiSupport,
    tag::{AnsiTag, default::Ansi},
};

pub struct AnsiSink<W: CharacterWriter> {
    pub(crate) writer: W,
    pub(crate) result: Result<(), W::Error>,
    pub(crate) support: AnsiSupport,
    pub(crate) last_tag: Option<Ansi>,
}

impl<W: CharacterWriter> AnsiSink<W> {
    fn append_internal<T2: Deref>(
        &mut self,
        grapheme: &gph,
        tag: Option<T2>,
    ) -> Result<(), W::Error>
    where
        T2::Target: AnsiTag,
    {
        write_intensity_delta(
            &mut self.writer,
            self.last_tag.as_ref(),
            tag.as_ref().map(|v| &**v),
        )?;

        write_blink_delta(
            &mut self.writer,
            self.last_tag.as_ref(),
            tag.as_ref().map(|v| &**v),
        )?;

        write_italic_delta(
            &mut self.writer,
            self.last_tag.as_ref(),
            tag.as_ref().map(|v| &**v),
        )?;

        write_concealed_delta(
            &mut self.writer,
            self.last_tag.as_ref(),
            tag.as_ref().map(|v| &**v),
        )?;

        write_strikethrough_delta(
            &mut self.writer,
            self.last_tag.as_ref(),
            tag.as_ref().map(|v| &**v),
        )?;

        write_underline_style_delta(
            &mut self.writer,
            self.last_tag.as_ref(),
            tag.as_ref().map(|v| &**v),
        )?;

        write_foreground_color_delta(
            &mut self.writer,
            self.support,
            self.last_tag.as_ref(),
            tag.as_ref().map(|v| &**v),
        )?;

        write_background_color_delta(
            &mut self.writer,
            self.support,
            self.last_tag.as_ref(),
            tag.as_ref().map(|v| &**v),
        )?;

        write_underline_color_delta(
            &mut self.writer,
            self.support,
            self.last_tag.as_ref(),
            tag.as_ref().map(|v| &**v),
        )?;

        write_hyperlink_delta(
            &mut self.writer,
            self.support,
            self.last_tag.as_ref(),
            tag.as_ref().map(|v| &**v),
        )?;

        self.writer.write_str(grapheme.as_str())?;

        self.last_tag = tag.map(Ansi::from_tag);

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum PlaintextError<E> {
    Renderable(#[from] RenderableError),
    Writer(E),
}

impl<W: CharacterWriter, T: Tag + Deref> TagSink<T> for AnsiSink<W>
where
    <T as Deref>::Target: AnsiTag + Sized,
{
    type Result = Result<W, PlaintextError<W::Error>>;

    fn append(&mut self, grapheme: &gph, tag: T) -> ControlFlow<()> {
        self.result = self.append_internal(grapheme, Some(tag));

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

    fn finalize(mut self) -> Self::Result {
        if let Err(e) = self.result {
            return Err(PlaintextError::Writer(e));
        }

        write_hyperlink_delta::<_, _, Ansi>(
            &mut self.writer,
            self.support,
            self.last_tag.as_ref(),
            None,
        )
        .map_err(|e| PlaintextError::Writer(e))?;

        self.writer
            .write_str("\x1b[0m")
            .map_err(|e| PlaintextError::Writer(e))?;

        Ok(self.writer)
    }
}
