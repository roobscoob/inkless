use core::ops::Deref;

use crate::grapheme::gph;

#[derive(Clone)]
pub struct Grapheme(String);

impl From<&gph> for Grapheme {
    fn from(value: &gph) -> Self {
        Self(String::from(value.as_str()))
    }
}

impl Deref for Grapheme {
    type Target = gph;

    fn deref(&self) -> &Self::Target {
        unsafe { gph::from_single_grapheme_str_unchecked(self.0.as_str()) }
    }
}
