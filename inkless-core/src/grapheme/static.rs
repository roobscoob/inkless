use core::{num::NonZero, ops::Deref};

use crate::grapheme::gph;

#[derive(Clone, Copy)]
pub enum StaticGrapheme<const GRAPHEME_WIDTH: usize = 7> {
    Overflowed,
    InBounds {
        length: NonZero<u8>,
        contents: [u8; GRAPHEME_WIDTH],
    },
}

impl<const GRAPHEME_WIDTH: usize> StaticGrapheme<GRAPHEME_WIDTH> {
    pub fn from_single_grapheme(grapheme: &gph) -> Self {
        if grapheme.len() > GRAPHEME_WIDTH {
            return StaticGrapheme::Overflowed;
        }

        if grapheme.len() > 254 {
            return StaticGrapheme::Overflowed;
        }

        let mut contents: [u8; GRAPHEME_WIDTH] = [0; GRAPHEME_WIDTH];

        contents[0..grapheme.len()].copy_from_slice(grapheme.as_str().as_bytes());

        StaticGrapheme::InBounds {
            length: unsafe { NonZero::new_unchecked(grapheme.len() as u8) },
            contents,
        }
    }

    pub fn from_single_grapheme_str(str: &str) -> Option<Self> {
        Self::from_str(str).next()
    }

    pub fn from_str(string: &str) -> impl Iterator<Item = Self> + '_ {
        gph::from_str(string).map(|v| Self::from_single_grapheme(v))
    }

    pub fn did_overflow(&self) -> bool {
        matches!(self, Self::Overflowed)
    }
}

const UNICODE_REPLACEMENT_CHARACTER: &'static gph =
    unsafe { gph::from_single_grapheme_str_unchecked("\u{FFFD}") };

impl<const GRAPHEME_WIDTH: usize> Deref for StaticGrapheme<GRAPHEME_WIDTH> {
    type Target = gph;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Overflowed => UNICODE_REPLACEMENT_CHARACTER,
            Self::InBounds { length, contents } => unsafe {
                gph::from_single_grapheme_utf8_unchecked(&contents[0..(length.get() - 1) as usize])
            },
        }
    }
}
