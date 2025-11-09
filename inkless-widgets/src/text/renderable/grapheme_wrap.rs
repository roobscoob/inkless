use inkless_core::{
    canvas::RenderBufferCanvas, grapheme::gph, render_position::RenderPosition, tag::Tag,
};
use inkless_macros::gph;

use crate::text::renderable::TextTag;

pub fn render_segment_grapheme_wrap<T1: Tag + Clone, T2: Tag + From<TextTag<T1>>>(
    text: &str,
    tag: &T1,
    canvas: &mut RenderBufferCanvas<'_, T2>,
    start: RenderPosition,
) {
    for grapheme in gph::from_str(text) {
        if grapheme == gph!("\n") {
            canvas.cursor_down().set_column(start.column());
            continue;
        }

        if !canvas.set_gph(grapheme, TextTag(tag.clone())) {
            // Try the same grapheme on the next line, same starting column.
            canvas.cursor_down().set_column(start.column());
            if !canvas.set_gph(grapheme, TextTag(tag.clone())) {
                // No vertical space either; give up on this segment.
                break;
            }
        }
    }
}
