use core::ops::Deref;

use crate::grapheme::gph;

#[derive(Clone, Copy)]
pub struct CharGrapheme(u8, [u8; 4]);

impl From<char> for CharGrapheme {
    fn from(value: char) -> Self {
        let mut v = [0u8; 4];

        let s = value.encode_utf8(&mut v);

        CharGrapheme(s.len() as u8, v)
    }
}

impl Deref for CharGrapheme {
    type Target = gph;

    fn deref(&self) -> &Self::Target {
        unsafe { gph::from_single_grapheme_utf8_unchecked(&self.1[0..self.0 as usize]) }
    }
}
