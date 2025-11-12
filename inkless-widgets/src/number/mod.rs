use core::ops::{Div, Rem, Sub};

use inkless_core::{
    canvas::Canvas,
    grapheme::gph,
    renderable::{Renderable, RenderableError},
    tag::Tag,
};
use num_traits::{NumCast, Zero};

use crate::number::{
    graphemes::{DIGIT_COUNT, DIGITS_UPPER, MINUS},
    separator::NumberSeparator,
    tag::NumberTag,
};

pub mod graphemes;
pub mod separator;
pub mod tag;

/// A renderable integer-like value with per-digit tagging and formatting.
///
/// `N` is your numeric type (`u32`, `i64`, etc.).
/// `T` is your tag type for the canvas.
/// `F` is a callback that produces a tag for each digit.
pub struct Number<N> {
    value: N,
    radix: u8,
    separator: Option<NumberSeparator>,
    prefix: Option<&'static str>,
    digits: &'static [&'static gph; DIGIT_COUNT],
}

impl<N> Number<N> {
    /// Create a new `Number` with a value and the default tag.
    ///
    /// `digit_tag` is called once per digit with:
    /// - `position`: 0 for the least-significant digit, 1 for the next, etc.
    /// - `digit`: the grapheme for that digit in the chosen radix and case.
    pub fn new(value: N) -> Self {
        Number {
            value,
            radix: 10,
            separator: None,
            prefix: None,
            digits: &DIGITS_UPPER,
        }
    }

    /// Set the radix/base in which the number will be rendered (2–36).
    /// Values outside that range are clamped.
    pub fn with_base(mut self, radix: u8) -> Self {
        self.radix = radix.max(2).min(36);
        self
    }

    /// Use lowercase alphabetic digits (`a`–`z`) for bases > 10.
    pub fn with_digits<D: Into<&'static [&'static gph; DIGIT_COUNT]>>(mut self, digits: D) -> Self {
        self.digits = digits.into();
        self
    }

    /// Configure grouping separators (e.g. spaces every 2 digits).
    ///
    /// `every` is the number of digits per group, counting from the right.
    /// `glyph` is the separator grapheme, `tag` is its tag.
    pub fn with_separator(mut self, every: usize, glyph: &'static gph) -> Self {
        assert!(every > 0, "separator group size must be > 0");
        self.separator = Some(NumberSeparator { every, glyph });
        self
    }

    /// Configure a prefix rendered after any negative sign and before digits.
    ///
    /// `glyphs` can represent multi-character prefixes such as `"0x"`, e.g.:
    /// `.with_prefix("0x", tag)`.
    pub fn with_prefix(mut self, glyphs: &'static str) -> Self {
        self.prefix = Some(glyphs);
        self
    }
}

impl<N, T: Tag + From<NumberTag>> Renderable<T> for Number<N>
where
    N: NumCast + Zero + Ord + Sub<Output = N> + Div<Output = N> + Rem<Output = N> + Clone,
    T: Tag,
{
    fn render_into<'buffer_reference>(
        &self,
        canvas: &mut dyn Canvas<T>,
    ) -> Result<(), RenderableError> {
        let start = canvas.get_position();

        // Normalize and cast base into `N`.
        let radix: N = NumCast::from(self.radix).expect("radix should fit into N");
        let zero = N::zero();

        // Extract sign and magnitude without needing `Signed`.
        let mut n = self.value.clone();
        let negative = n < zero;

        // Convert absolute value into base-`radix` digits, least-significant first.
        let mut digits: [u8; 128] = [0; 128];
        let mut len = 0usize;

        if n == zero {
            digits[0] = 0;
            len = 1;
        } else {
            while n != zero && len < digits.len() {
                let r = n.clone() % radix.clone();
                let q = n.clone() / radix.clone();

                let mut d_i8: i8 = r
                    .to_i8()
                    .expect("digit remainder should always fit into u8 for radix <= 36");

                if negative {
                    d_i8 = -d_i8;
                }

                digits[len] = d_i8 as u8;
                len += 1;
                n = q;
            }
        }

        if len == 0 {
            // Nothing to render; just succeed.
            return Ok(());
        }

        // 1. Render sign if needed.
        if negative {
            if !canvas.set_gph(MINUS, NumberTag::Minus.into()) {
                canvas.cursor_down().set_column(start.column());
                canvas.set_gph(MINUS, NumberTag::Minus.into());
            }
        }

        // 2. Render prefix, if present.
        if let Some(prefix) = &self.prefix {
            for (i, g) in gph::from_str(prefix).enumerate() {
                if !canvas.set_gph(g, NumberTag::Prefix { index: i }.into()) {
                    canvas.cursor_down().set_column(start.column());
                    canvas.set_gph(g, NumberTag::Prefix { index: i }.into());
                }
            }
        }

        // 3. Render digits + separators, left to right (MSD → LSD).
        //
        // digits[0] = LSD, digits[len-1] = MSD
        // position_from_right == index into `digits`.
        let sep = self.separator.as_ref();
        let mut sep_i = 0;

        for (i, j) in (0..len).rev().enumerate() {
            if let Some(sep_cfg) = sep {
                // Groups counted from the right:
                // position_from_right = j
                // Insert a separator before this digit whenever
                // we've already emitted an integral number of groups
                // of `every` digits from the right.
                //
                // Example (len = 6, every = 2):
                //  positions_from_right: [5,4,3,2,1,0] (for digits j=5..0)
                //  output: d5 d4 | d3 d2 | d1 d0 → "10 92 81" style.
                if j != len - 1 {
                    let prev_pos_from_right = j + 1;
                    if prev_pos_from_right % sep_cfg.every == 0 {
                        if !canvas
                            .set_gph(sep_cfg.glyph, NumberTag::Separator { index: sep_i }.into())
                        {
                            canvas.cursor_down().set_column(start.column());
                            canvas.set_gph(
                                sep_cfg.glyph,
                                NumberTag::Separator { index: sep_i }.into(),
                            );
                        }

                        sep_i += 1;
                    }
                }
            }

            let glyph = self.digits[digits[j] as usize];

            if !canvas.set_gph(
                glyph,
                NumberTag::Digit {
                    index: i,
                    value: digits[j],
                }
                .into(),
            ) {
                canvas.cursor_down().set_column(start.column());
                canvas.set_gph(
                    glyph,
                    NumberTag::Digit {
                        index: i,
                        value: digits[j],
                    }
                    .into(),
                );
            }
        }

        Ok(())
    }
}
