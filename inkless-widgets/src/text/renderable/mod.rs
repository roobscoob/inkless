pub mod clip;
pub mod ellipsis;
pub mod grapheme_wrap;
pub mod word_wrap;

use inkless_core::{
    canvas::{Canvas, into::IntoCanvas},
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
