use inkless_core::tag::{Tag, untagged::Untagged};

use crate::text::overflow::EllipsisPosition;

pub enum TextTag<T1: Tag + Clone> {
    Segment(T1),

    Ellipsis(EllipsisPosition),
}

impl<T1: Tag + Clone> Tag for TextTag<T1> {}

impl<T1: Tag + Clone> From<T1> for TextTag<T1> {
    fn from(value: T1) -> Self {
        Self::Segment(value)
    }
}

impl<T1: Tag + Clone> From<TextTag<T1>> for Untagged {
    fn from(_: TextTag<T1>) -> Self {
        Untagged
    }
}
