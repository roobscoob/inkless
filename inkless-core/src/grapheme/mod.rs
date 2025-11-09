use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::canvas::AmbiguityPolicy;

pub mod char;
pub mod r#static;

#[cfg(feature = "alloc")]
pub mod grapheme;

#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub struct gph(str);

impl gph {
    pub const unsafe fn from_single_grapheme_str_unchecked<'a>(s: &'a str) -> &'a gph {
        // SAFETY:
        // - X is #[repr(transparent)] over `str`, so it has the same layout and alignment.
        // - `s` is a valid reference for the lifetime of the returned &X.
        unsafe { &*(s as *const str as *const gph) }
    }

    pub const unsafe fn from_single_grapheme_utf8_unchecked<'a>(s: &'a [u8]) -> &'a gph {
        // SAFETY:
        // - X is #[repr(transparent)] over `str`, so it has the same layout and alignment.
        // - `s` is a valid reference for the lifetime of the returned &X.
        unsafe { &*(core::str::from_utf8_unchecked(s) as *const str as *const gph) }
    }

    pub fn from_single_grapheme_str<'a>(str: &'a str) -> Option<&'a gph> {
        Self::from_str(str).next()
    }

    pub fn from_str<'a>(string: &'a str) -> impl Iterator<Item = &'a gph> {
        string
            .graphemes(true)
            .map(|grapheme| unsafe { Self::from_single_grapheme_str_unchecked(grapheme) })
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self, policy: AmbiguityPolicy) -> usize {
        match policy {
            AmbiguityPolicy::Standard => self.width_normal(),
            AmbiguityPolicy::Wide => self.width_cjk(),
        }
    }

    pub fn width_normal(&self) -> usize {
        self.0.width()
    }

    pub fn width_cjk(&self) -> usize {
        self.0.width_cjk()
    }
}
