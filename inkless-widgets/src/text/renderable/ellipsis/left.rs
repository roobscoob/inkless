use inkless_core::{canvas::RenderBufferCanvas, render_position::RenderPosition, tag::Tag};
use inkless_macros::gph;

use crate::text::renderable::{
    TextTag,
    ellipsis::{
        count_graphemes, draw_ellipsis_plus_suffix, draw_line_full, ellipsis_plus_suffix_fits,
        line_fits_without_ellipsis,
    },
};

pub fn render_segment_ellipsis_left<T1: Tag + Clone, T2: Tag + From<TextTag<T1>>>(
    text: &str,
    tag: &T1,
    overflow_tag: Option<T1>,
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

        if line_fits_without_ellipsis(line, canvas, line_start) {
            draw_line_full(line, tag, canvas);
            continue;
        }

        // Binary search the longest suffix that fits with a leading ellipsis.
        let mut lo = 0usize;
        let mut hi = total;

        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if ellipsis_plus_suffix_fits(line, total, mid, canvas, line_start, ell) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        let best_suffix = lo;

        // If even ellipsis alone can't be placed, bail.
        if best_suffix == 0 && !ellipsis_plus_suffix_fits(line, total, 0, canvas, line_start, ell) {
            continue;
        }

        canvas.set_position(line_start);
        draw_ellipsis_plus_suffix(line, total, best_suffix, tag, overflow_tag.clone(), canvas);
    }
}
