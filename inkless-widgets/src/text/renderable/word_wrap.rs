use inkless_core::{canvas::Canvas, grapheme::gph, render_position::RenderPosition, tag::Tag};
use inkless_macros::gph;
use unicode_segmentation::UnicodeSegmentation;

use crate::text::renderable::TextTag;

fn chunk_fits_on_current_line<T: Tag>(chunk: &str, canvas: &mut dyn Canvas<T>) -> bool {
    let c = canvas.get_position();

    for grapheme in gph::from_str(chunk) {
        if grapheme == gph!("\n") {
            // Treat an explicit newline as "ok, we can break here".
            canvas.set_position(c);
            return true;
        }

        // Ask the buffer if this grapheme would fit entirely at this position.
        if !canvas.can_set_gph(grapheme) {
            canvas.set_position(c);
            return false;
        }
    }

    canvas.set_position(c);
    true
}

pub fn render_segment_word_wrap<T1: Tag + Clone, T2: Tag, T3: Tag + From<TextTag<T1, T2>>>(
    text: &str,
    tag: &T1,
    canvas: &mut dyn Canvas<T3>,
    start: RenderPosition,
) {
    for chunk in text.split_word_bounds() {
        // Handle explicit newlines as their own "chunk".
        if chunk == "\n" || chunk == "\r\n" {
            canvas.cursor_down().set_column(start.column());
            continue;
        }

        if chunk_fits_on_current_line(chunk, canvas) {
            for grapheme in gph::from_str(chunk) {
                canvas.set_gph(grapheme, TextTag::Segment(tag.clone()).into());
            }

            continue;
        }

        let pre_move = canvas.get_position();

        canvas.cursor_down().set_column(start.column());

        if chunk_fits_on_current_line(chunk, canvas) {
            for grapheme in gph::from_str(chunk) {
                canvas.set_gph(grapheme, TextTag::Segment(tag.clone()).into());
            }

            continue;
        }

        canvas.set_position(pre_move);

        for grapheme in gph::from_str(chunk) {
            if !canvas.set_gph(grapheme, TextTag::Segment(tag.clone()).into()) {
                canvas.cursor_down().set_column(start.column());
                canvas.set_gph(grapheme, TextTag::Segment(tag.clone()).into());
            }
        }
    }
}
