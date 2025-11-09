use inkless_core::tag::{Tag, untagged::Untagged};

#[derive(Clone, Copy, Debug)]
pub enum NumberTag {
    Minus,
    Prefix { index: usize },
    Digit { index: usize, value: u8 },
    Separator { index: usize },
}

impl Tag for NumberTag {}

impl From<NumberTag> for Untagged {
    fn from(_: NumberTag) -> Self {
        Untagged
    }
}
