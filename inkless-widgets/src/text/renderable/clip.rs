use core::ops::ControlFlow;

use inkless_core::{canvas::Canvas, grapheme::gph, render_position::RenderPosition, tag::Tag};
use inkless_macros::gph;

use crate::text::renderable::TextTag;

/// Simple "draw until we can't" behaviour.
/// - Newlines move to the next row at the original start column.
/// - Horizontal overflow just stops rendering further graphemes.
pub fn render_segment_clip<T1: Tag + Clone, T2: Tag, T3: Tag + From<TextTag<T1, T2>>>(
    text: &str,
    tag: &T1,
    canvas: &mut dyn Canvas<T3>,
    start: RenderPosition,
) -> ControlFlow<()> {
    for grapheme in gph::from_str(text) {
        if grapheme == gph!("\n") {
            canvas.cursor_down().set_column(start.column());
            continue;
        }

        // If this returns false, we silently drop the rest.
        if !canvas.set_gph(grapheme, TextTag::Segment(tag.clone()).into()) {
            return ControlFlow::Break(());
        }
    }

    ControlFlow::Continue(())
}
