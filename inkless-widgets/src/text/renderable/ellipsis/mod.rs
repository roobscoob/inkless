pub mod center;
pub mod left;
pub mod right;

use inkless_core::{
    canvas::RenderBufferCanvas, grapheme::gph, render_position::RenderPosition, tag::Tag,
};
use inkless_macros::gph;

use crate::text::{
    overflow::EllipsisPosition,
    renderable::{
        TextTag,
        ellipsis::{
            center::render_segment_ellipsis_center, left::render_segment_ellipsis_left,
            right::render_segment_ellipsis_right,
        },
    },
};

pub fn render_segment_ellipsis<T1: Tag + Clone, T2: Tag + From<TextTag<T1>>>(
    text: &str,
    tag: &T1,
    canvas: &mut RenderBufferCanvas<'_, T2>,
    start: RenderPosition,
    position: EllipsisPosition,
) {
    match position {
        EllipsisPosition::Right => render_segment_ellipsis_right(text, tag, canvas, start),
        EllipsisPosition::Left => render_segment_ellipsis_left(text, tag, canvas, start),
        EllipsisPosition::Center => render_segment_ellipsis_center(text, tag, canvas, start),
    }
}

pub(self) fn count_graphemes(text: &str) -> usize {
    let mut n = 0;
    for _ in gph::from_str(text) {
        n += 1;
    }
    n
}

pub(self) fn line_fits_without_ellipsis<T: Tag>(
    line: &str,
    canvas: &mut RenderBufferCanvas<'_, T>,
    start: RenderPosition,
) -> bool {
    let saved = canvas.get_position();
    canvas.set_position(start);

    for grapheme in gph::from_str(line) {
        if !canvas.can_set_gph(grapheme) {
            canvas.set_position(saved);
            return false;
        }
    }

    canvas.set_position(saved);
    true
}

pub(self) fn prefix_plus_ellipsis_fits<T: Tag>(
    line: &str,
    prefix_len: usize,
    canvas: &mut RenderBufferCanvas<'_, T>,
    start: RenderPosition,
    ell: &gph,
) -> bool {
    let saved = canvas.get_position();
    canvas.set_position(start);

    let mut idx = 0;
    for grapheme in gph::from_str(line) {
        if idx >= prefix_len {
            break;
        }

        if !canvas.can_set_gph(grapheme) {
            canvas.set_position(saved);
            return false;
        }

        idx += 1;
    }

    // Now try to place the ellipsis.
    if !canvas.can_set_gph(ell) {
        canvas.set_position(saved);
        return false;
    }

    canvas.set_position(saved);
    true
}

pub(self) fn ellipsis_plus_suffix_fits<T: Tag>(
    line: &str,
    total_graphemes: usize,
    suffix_len: usize,
    canvas: &mut RenderBufferCanvas<'_, T>,
    start: RenderPosition,
    ell: &gph,
) -> bool {
    debug_assert!(suffix_len <= total_graphemes);

    let saved = canvas.get_position();
    canvas.set_position(start);

    // First: ellipsis.
    if !canvas.can_set_gph(ell) {
        canvas.set_position(saved);
        return false;
    }

    let skip = total_graphemes.saturating_sub(suffix_len);
    let mut idx = 0;

    for grapheme in gph::from_str(line) {
        if idx >= skip {
            if !canvas.can_set_gph(grapheme) {
                canvas.set_position(saved);
                return false;
            }
        }
        idx += 1;
    }

    canvas.set_position(saved);
    true
}

/// Test: prefix[0..prefix_len] + ellipsis + suffix[last suffix_len] fits.
pub(self) fn center_candidate_fits<T: Tag>(
    line: &str,
    total_graphemes: usize,
    prefix_len: usize,
    suffix_len: usize,
    canvas: &mut RenderBufferCanvas<'_, T>,
    start: RenderPosition,
    ell: &gph,
) -> bool {
    debug_assert!(prefix_len + suffix_len <= total_graphemes);

    let saved = canvas.get_position();
    canvas.set_position(start);

    // 1. Prefix.
    let mut idx = 0;
    for grapheme in gph::from_str(line) {
        if idx >= prefix_len {
            break;
        }
        if !canvas.can_set_gph(grapheme) {
            canvas.set_position(saved);
            return false;
        }
        idx += 1;
    }

    // 2. Ellipsis.
    if !canvas.can_set_gph(ell) {
        canvas.set_position(saved);
        return false;
    }

    // 3. Suffix.
    let suffix_start = total_graphemes.saturating_sub(suffix_len);
    idx = 0;
    for grapheme in gph::from_str(line) {
        if idx >= suffix_start {
            if !canvas.can_set_gph(grapheme) {
                canvas.set_position(saved);
                return false;
            }
        }
        idx += 1;
    }

    canvas.set_position(saved);
    true
}

pub(self) fn draw_line_full<T1: Tag + Clone, T2: Tag + From<TextTag<T1>>>(
    line: &str,
    tag: &T1,
    canvas: &mut RenderBufferCanvas<'_, T2>,
) {
    for grapheme in gph::from_str(line) {
        if !canvas.set_gph(grapheme, TextTag::Segment(tag.clone())) {
            break;
        }
    }
}

pub(self) fn draw_prefix_plus_ellipsis<T1: Tag + Clone, T2: Tag + From<TextTag<T1>>>(
    line: &str,
    prefix_len: usize,
    tag: &T1,
    canvas: &mut RenderBufferCanvas<'_, T2>,
) {
    let ell = gph!("…");

    let mut idx = 0;
    for grapheme in gph::from_str(line) {
        if idx >= prefix_len {
            break;
        }

        if !canvas.set_gph(grapheme, TextTag::Segment(tag.clone())) {
            return;
        }

        idx += 1;
    }

    let _ = canvas.set_gph(ell, TextTag::Ellipsis(EllipsisPosition::Right));
}

pub(self) fn draw_ellipsis_plus_suffix<T1: Tag + Clone, T2: Tag + From<TextTag<T1>>>(
    line: &str,
    total_graphemes: usize,
    suffix_len: usize,
    tag: &T1,
    canvas: &mut RenderBufferCanvas<'_, T2>,
) {
    let ell = gph!("…");
    let _ = canvas.set_gph(ell, TextTag::Ellipsis(EllipsisPosition::Left));

    let skip = total_graphemes.saturating_sub(suffix_len);
    let mut idx = 0;

    for grapheme in gph::from_str(line) {
        if idx >= skip {
            if !canvas.set_gph(grapheme, TextTag::Segment(tag.clone())) {
                return;
            }
        }
        idx += 1;
    }
}

pub(super) fn draw_center<T1: Tag + Clone, T2: Tag + From<TextTag<T1>>>(
    line: &str,
    total_graphemes: usize,
    prefix_len: usize,
    suffix_len: usize,
    tag: &T1,
    canvas: &mut RenderBufferCanvas<'_, T2>,
) {
    let ell = gph!("…");

    // Prefix
    let mut idx = 0;
    for grapheme in gph::from_str(line) {
        if idx >= prefix_len {
            break;
        }

        if !canvas.set_gph(grapheme, TextTag::Segment(tag.clone())) {
            return;
        }

        idx += 1;
    }

    // Ellipsis
    if !canvas.set_gph(ell, TextTag::Ellipsis(EllipsisPosition::Center)) {
        return;
    }

    // Suffix
    let suffix_start = total_graphemes.saturating_sub(suffix_len);
    idx = 0;

    for grapheme in gph::from_str(line) {
        if idx >= suffix_start {
            if !canvas.set_gph(grapheme, TextTag::Segment(tag.clone())) {
                return;
            }
        }
        idx += 1;
    }
}
