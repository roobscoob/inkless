pub mod clip;
pub mod ellipsis;
pub mod grapheme_wrap;
pub mod word_wrap;

use inkless_core::{
    canvas::{Canvas, into::IntoCanvas, summary::CanvasSummary},
    grapheme::gph,
    render_position::RenderPosition,
    renderable::{Renderable, RenderableError},
    tag::Tag,
};

use crate::text::{
    Text,
    overflow::Overflow,
    renderable::{
        clip::render_segment_clip, ellipsis::render_segment_ellipsis,
        grapheme_wrap::render_segment_grapheme_wrap, word_wrap::render_segment_word_wrap,
    },
    segment::{SegmentStore, SegmentStoreFetch, TextSegment},
    tag::TextTag,
};

struct OverflowError;

impl core::error::Error for OverflowError {
    fn description(&self) -> &str {
        "OverflowError: The text overflowed the canvas, and `Overflow::Error` was selected."
    }
}

impl core::fmt::Display for OverflowError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(
            "OverflowError: The text overflowed the canvas, and `Overflow::Error` was selected.",
        )
    }
}

impl core::fmt::Debug for OverflowError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("OverflowError")
    }
}

impl<T1: Tag + Clone, S: SegmentStore<T1> + SegmentStoreFetch<T1, T3>, T3: Tag>
    Renderable<TextTag<T1, T3>> for Text<S, T1>
{
    fn render_into<'buffer_reference>(
        &self,
        canvas: &mut dyn Canvas<TextTag<T1, T3>>,
    ) -> Result<(), RenderableError> {
        let start = canvas.get_position();

        for i in 0..self.segments.len() {
            let segment = match self.segments.get(i) {
                Some(s) => s,
                None => continue,
            };

            match segment {
                TextSegment::Renderable(r) => {
                    // Delegate to nested renderable as before.
                    IntoCanvas::new(canvas).write(r)?;
                }
                TextSegment::Segment(text, tag) => {
                    match &self.overflow_behavior {
                        Overflow::Clip => {
                            // we don't care when it overflows
                            let _ = render_segment_clip(text, tag, canvas, start);
                        }

                        Overflow::GraphemeWrap => {
                            render_segment_grapheme_wrap(text, tag, canvas, start);
                        }

                        Overflow::WordWrap => {
                            render_segment_word_wrap(text, tag, canvas, start);
                        }

                        Overflow::Ellipsis(position) => {
                            render_segment_ellipsis(text, tag, canvas, start, *position);
                        }

                        Overflow::Error => {
                            if render_segment_clip(text, tag, canvas, start).is_break() {
                                return Err(RenderableError::of(|a| {
                                    a.handle_error(&OverflowError);
                                }));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl<T1: Tag + Clone + Default, S: SegmentStore<T1> + SegmentStoreFetch<T1, T1>> Renderable<T1>
    for Text<S, T1>
{
    fn render_into<'buffer_reference>(
        &self,
        canvas: &mut dyn Canvas<T1>,
    ) -> Result<(), RenderableError> {
        let start = canvas.get_position();

        for i in 0..self.segments.len() {
            let segment = match self.segments.get(i) {
                Some(s) => s,
                None => continue,
            };

            match segment {
                TextSegment::Renderable(r) => {
                    // Delegate to nested renderable as before.
                    canvas.write(r)?;
                }
                TextSegment::Segment(text, tag) => {
                    match &self.overflow_behavior {
                        Overflow::Clip => {
                            // we don't care when it overflows
                            let _ = render_segment_clip(
                                text,
                                tag,
                                &mut FlattenCanvas::new(canvas),
                                start,
                            );
                        }

                        Overflow::GraphemeWrap => {
                            render_segment_grapheme_wrap(
                                text,
                                tag,
                                &mut FlattenCanvas::new(canvas),
                                start,
                            );
                        }

                        Overflow::WordWrap => {
                            render_segment_word_wrap(
                                text,
                                tag,
                                &mut FlattenCanvas::new(canvas),
                                start,
                            );
                        }

                        Overflow::Ellipsis(position) => {
                            render_segment_ellipsis(
                                text,
                                tag,
                                &mut FlattenCanvas::new(canvas),
                                start,
                                *position,
                            );
                        }

                        Overflow::Error => {
                            if render_segment_clip(
                                text,
                                tag,
                                &mut FlattenCanvas::new(canvas),
                                start,
                            )
                            .is_break()
                            {
                                return Err(RenderableError::of(|a| {
                                    a.handle_error(&OverflowError);
                                }));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

pub struct FlattenCanvas<'a, Tt: Tag> {
    pub(crate) inner: &'a mut dyn Canvas<Tt>,
}

impl<'a, Tt: Tag> FlattenCanvas<'a, Tt> {
    pub fn new(inner: &'a mut dyn Canvas<Tt>) -> Self {
        Self { inner }
    }
}

impl<'a, Ot: Tag + Clone + Default> Canvas<TextTag<Ot, Ot>> for FlattenCanvas<'a, Ot> {
    fn write<'b>(
        &'b mut self,
        renderable: &dyn Renderable<TextTag<Ot, Ot>>,
    ) -> Result<CanvasSummary, RenderableError> {
        let start = self.inner.get_position();
        renderable.render_into(self)?;
        let end = self.inner.get_position();

        Ok(CanvasSummary {
            start_position: start,
            end_position: end,
        })
    }

    fn set_char(&mut self, ch: char, tag: TextTag<Ot, Ot>) -> bool {
        self.inner.set_char(
            ch,
            match tag {
                TextTag::Component(c) => c,
                TextTag::Segment(s) => s,
                TextTag::Ellipsis(_) => Default::default(),
            },
        )
    }

    fn set_gph(&mut self, v: &gph, tag: TextTag<Ot, Ot>) -> bool {
        self.inner.set_gph(
            v,
            match tag {
                TextTag::Component(c) => c,
                TextTag::Segment(s) => s,
                TextTag::Ellipsis(_) => Default::default(),
            },
        )
    }

    fn can_set_gph(&mut self, v: &gph) -> bool {
        // Width is independent of tag, so just forward.
        self.inner.can_set_gph(v)
    }

    fn get_start_position(&self) -> RenderPosition {
        self.inner.get_start_position()
    }

    fn get_position(&self) -> RenderPosition {
        self.inner.get_position()
    }

    fn set_position(&mut self, position: RenderPosition) -> &mut dyn Canvas<TextTag<Ot, Ot>> {
        self.inner.set_position(position);
        self
    }

    fn set_column(&mut self, column: usize) -> &mut dyn Canvas<TextTag<Ot, Ot>> {
        self.inner.set_column(column);
        self
    }

    fn set_line(&mut self, line: usize) -> &mut dyn Canvas<TextTag<Ot, Ot>> {
        self.inner.set_line(line);
        self
    }

    fn cursor_down(&mut self) -> &mut dyn Canvas<TextTag<Ot, Ot>> {
        self.inner.cursor_down();
        self
    }

    fn cursor_down_by(&mut self, count: usize) -> &mut dyn Canvas<TextTag<Ot, Ot>> {
        self.inner.cursor_down_by(count);
        self
    }

    fn try_cursor_up(&mut self) -> bool {
        self.inner.try_cursor_up()
    }

    fn try_cursor_up_by(&mut self, count: usize) -> bool {
        self.inner.try_cursor_up_by(count)
    }

    fn cursor_up(&mut self) -> &mut dyn Canvas<TextTag<Ot, Ot>> {
        self.inner.cursor_up();
        self
    }

    fn cursor_up_by(&mut self, count: usize) -> &mut dyn Canvas<TextTag<Ot, Ot>> {
        self.inner.cursor_up_by(count);
        self
    }

    fn try_cursor_left(&mut self) -> bool {
        self.inner.try_cursor_left()
    }

    fn try_cursor_left_by(&mut self, count: usize) -> bool {
        self.inner.try_cursor_left_by(count)
    }

    fn cursor_left(&mut self) -> &mut dyn Canvas<TextTag<Ot, Ot>> {
        self.inner.cursor_left();
        self
    }

    fn cursor_left_by(&mut self, count: usize) -> &mut dyn Canvas<TextTag<Ot, Ot>> {
        self.inner.cursor_left_by(count);
        self
    }

    fn cursor_right(&mut self) -> &mut dyn Canvas<TextTag<Ot, Ot>> {
        self.inner.cursor_right();
        self
    }

    fn cursor_right_by(&mut self, count: usize) -> &mut dyn Canvas<TextTag<Ot, Ot>> {
        self.inner.cursor_right_by(count);
        self
    }
}
