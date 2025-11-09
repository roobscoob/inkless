use inkless_core::{canvas::RenderBufferCanvas, render_position::RenderPosition, tag::Tag};
use inkless_macros::gph;

use crate::text::renderable::{
    TextTag,
    ellipsis::{
        count_graphemes, draw_line_full, draw_prefix_plus_ellipsis, line_fits_without_ellipsis,
        prefix_plus_ellipsis_fits,
    },
};

pub fn render_segment_ellipsis_right<T1: Tag + Clone, T2: Tag + From<TextTag<T1>>>(
    text: &str,
    tag: &T1,
    canvas: &mut RenderBufferCanvas<'_, T2>,
    start: RenderPosition,
) {
    let ell = gph!("…");

    for (i, line) in text.split('\n').enumerate() {
        if i > 0 {
            canvas.cursor_down().set_column(start.column());
        }

        let line_start = canvas.get_position();
        let total = count_graphemes(line);

        // If the whole line fits, just draw it.
        if line_fits_without_ellipsis(line, canvas, line_start) {
            draw_line_full(line, tag, canvas);
            continue;
        }

        // Binary search the longest prefix that fits with an ellipsis.
        let mut lo = 0usize;
        let mut hi = total;

        // We also want to handle the "only ellipsis" case, so 0 is allowed.
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if prefix_plus_ellipsis_fits(line, mid, canvas, line_start, ell) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        let best_prefix = lo;

        // If even ellipsis alone doesn't fit, we give up on this line.
        if best_prefix == 0 && !prefix_plus_ellipsis_fits(line, 0, canvas, line_start, ell) {
            continue;
        }

        canvas.set_position(line_start);
        draw_prefix_plus_ellipsis(line, best_prefix, tag, canvas);
    }
}
