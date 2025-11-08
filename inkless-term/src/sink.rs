use core::ops::ControlFlow;

use inkless_core::{grapheme::gph, tag::sink::TagSink, writer::character::CharacterWriter};

use crate::{
    delta::{
        write_background_color_delta, write_blink_delta, write_concealed_delta,
        write_foreground_color_delta, write_hyperlink_delta, write_intensity_delta,
        write_italic_delta, write_strikethrough_delta, write_underline_color_delta,
        write_underline_style_delta,
    },
    support::AnsiSupport,
    tag::AnsiTag,
};

pub struct Ansi<W: CharacterWriter, T: AnsiTag> {
    pub(crate) writer: W,
    pub(crate) result: Result<(), W::Error>,
    pub(crate) support: AnsiSupport,
    pub(crate) last_tag: Option<T>,
}

impl<W: CharacterWriter, T: AnsiTag> Ansi<W, T> {
    fn append_internal(&mut self, grapheme: &gph, tag: Option<T>) -> Result<(), W::Error> {
        write_intensity_delta(&mut self.writer, self.last_tag.as_ref(), tag.as_ref())?;
        write_blink_delta(&mut self.writer, self.last_tag.as_ref(), tag.as_ref())?;
        write_italic_delta(&mut self.writer, self.last_tag.as_ref(), tag.as_ref())?;
        write_concealed_delta(&mut self.writer, self.last_tag.as_ref(), tag.as_ref())?;
        write_strikethrough_delta(&mut self.writer, self.last_tag.as_ref(), tag.as_ref())?;
        write_underline_style_delta(&mut self.writer, self.last_tag.as_ref(), tag.as_ref())?;

        write_foreground_color_delta(
            &mut self.writer,
            self.support,
            self.last_tag.as_ref(),
            tag.as_ref(),
        )?;

        write_background_color_delta(
            &mut self.writer,
            self.support,
            self.last_tag.as_ref(),
            tag.as_ref(),
        )?;

        write_underline_color_delta(
            &mut self.writer,
            self.support,
            self.last_tag.as_ref(),
            tag.as_ref(),
        )?;

        write_hyperlink_delta(
            &mut self.writer,
            self.support,
            self.last_tag.as_ref(),
            tag.as_ref(),
        )?;

        self.writer.write_str(grapheme.as_str())?;

        self.last_tag = tag;

        Ok(())
    }
}

impl<W: CharacterWriter, T: AnsiTag> TagSink<T> for Ansi<W, T> {
    type Result = Result<W, W::Error>;

    fn append(&mut self, grapheme: &gph) -> ControlFlow<()> {
        self.result = self.append_internal(grapheme, None);

        if self.result.is_err() {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }

    fn append_tagged(&mut self, grapheme: &gph, tag: T) -> ControlFlow<()> {
        self.result = self.append_internal(grapheme, Some(tag));

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
            return Err(e);
        }

        write_hyperlink_delta(&mut self.writer, self.support, self.last_tag.as_ref(), None)?;

        self.writer.write_str("\x1b[0m")?;

        Ok(self.writer)
    }
}
