use inkless_core::{canvas::RenderBufferCanvas, render_position::RenderPosition, tag::Tag};
use inkless_macros::gph;

use crate::text::renderable::{
    TextTag,
    ellipsis::{
        center_candidate_fits, count_graphemes, draw_center, draw_line_full,
        line_fits_without_ellipsis, prefix_plus_ellipsis_fits,
    },
};

pub fn render_segment_ellipsis_center<T1: Tag + Clone, T2: Tag + From<TextTag<T1>>>(
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

        if line_fits_without_ellipsis(line, canvas, line_start) {
            draw_line_full(line, tag, canvas);
            continue;
        }

        // Step 1: get the maximal prefix that fits with an ellipsis (right-style).
        let mut lo = 0usize;
        let mut hi = total;

        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if prefix_plus_ellipsis_fits(line, mid, canvas, line_start, ell) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        let mut prefix_len = lo;
        let mut suffix_len = 0usize;

        // If we can't even place the ellipsis at all, bail on this line.
        if prefix_len == 0 && !prefix_plus_ellipsis_fits(line, 0, canvas, line_start, ell) {
            continue;
        }

        // Step 2: "rebalance": trade some prefix into suffix while it still fits.
        //
        // We repeatedly try to move one grapheme from left-side to right-side:
        //   (prefix_len, suffix_len) -> (prefix_len - 1, suffix_len + 1)
        // as long as the candidate fits.
        //
        // This tends to pull the ellipsis toward the middle while keeping
        // as much total visible content as possible.
        while prefix_len > suffix_len && prefix_len + suffix_len < total {
            let candidate_prefix = prefix_len - 1;
            let candidate_suffix = suffix_len + 1;

            if center_candidate_fits(
                line,
                total,
                candidate_prefix,
                candidate_suffix,
                canvas,
                line_start,
                ell,
            ) {
                prefix_len = candidate_prefix;
                suffix_len = candidate_suffix;
            } else {
                break;
            }
        }

        canvas.set_position(line_start);
        draw_center(line, total, prefix_len, suffix_len, tag, canvas);
    }
}
