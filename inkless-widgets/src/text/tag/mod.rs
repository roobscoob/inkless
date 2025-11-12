use inkless_core::tag::Tag;

use crate::text::overflow::EllipsisPosition;

pub enum TextTag<T1: Tag + Clone, T2: Tag> {
    Segment(T1),
    Component(T2),

    Ellipsis(EllipsisPosition),
}

impl<T1: Tag + Clone, T2: Tag> Tag for TextTag<T1, T2> {}

impl<T1: Tag + Clone, T2: Tag> From<T2> for TextTag<T1, T2> {
    fn from(value: T2) -> Self {
        Self::Component(value)
    }
}
