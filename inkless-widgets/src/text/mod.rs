use inkless_core::{
    renderable::Renderable,
    tag::{Tag, untagged::Untagged},
};

use crate::text::{
    overflow::{EllipsisPosition, NoOverflowTag, Overflow, OverflowTag, SomeOverflowTag},
    segment::{
        SegmentStore,
        recursive::{RecursiveSegmentStoreNone, RecursiveSegmentStoreSomeSegment},
    },
};

pub mod overflow;
pub mod renderable;
pub mod segment;

pub struct Text<S, O: OverflowTag> {
    segments: S,
    overflow_behavior: Overflow<O>,
}

impl<S, O: OverflowTag> Text<S, O> {
    fn from_store(store: S) -> Self {
        Self {
            segments: store,
            overflow_behavior: Default::default(),
        }
    }
}

impl Text<RecursiveSegmentStoreNone, NoOverflowTag<Untagged>> {
    pub fn empty<T: Tag>() -> Text<RecursiveSegmentStoreNone, NoOverflowTag<T>> {
        Text::from_store(RecursiveSegmentStoreNone)
    }

    pub fn of_tagged<T: Tag>(
        text: &'static str,
        tag: T,
    ) -> Text<RecursiveSegmentStoreSomeSegment<T, RecursiveSegmentStoreNone>, NoOverflowTag<T>>
    {
        Text {
            segments: RecursiveSegmentStoreSomeSegment {
                segment: text,
                tag,
                tail: RecursiveSegmentStoreNone,
            },
            overflow_behavior: Overflow::default(),
        }
    }

    pub fn of<T: Tag>(
        text: &'static str,
    ) -> Text<RecursiveSegmentStoreSomeSegment<T, RecursiveSegmentStoreNone>, NoOverflowTag<T>>
    where
        T: Default,
    {
        Text {
            segments: RecursiveSegmentStoreSomeSegment {
                segment: text,
                tag: Default::default(),
                tail: RecursiveSegmentStoreNone,
            },
            overflow_behavior: Overflow::default(),
        }
    }
}

impl<T: Tag + Default, O: OverflowTag> From<&'static str>
    for Text<RecursiveSegmentStoreSomeSegment<T, RecursiveSegmentStoreNone>, O>
{
    fn from(value: &'static str) -> Self {
        Self::from_store(RecursiveSegmentStoreSomeSegment {
            segment: value,
            tag: T::default(),
            tail: RecursiveSegmentStoreNone,
        })
    }
}

pub trait TextWithRenderable<Ta: Tag, C, S, O: OverflowTag> {
    fn with_component<Tb: Tag>(self, value: C) -> Text<S::WithRenderable<C>, O>
    where
        S: SegmentStore<Ta, Tb>,
        C: Renderable<Tb>;
}

impl<Ta: Tag, C, S, O: OverflowTag> TextWithRenderable<Ta, C, S, O> for Text<S, O> {
    fn with_component<Tb: Tag>(self, value: C) -> Text<S::WithRenderable<C>, O>
    where
        S: SegmentStore<Ta, Tb>,
        C: Renderable<Tb>,
    {
        Text {
            segments: self.segments.with_renderable(value),
            overflow_behavior: self.overflow_behavior,
        }
    }
}

pub trait WithTagged<T2: Tag, O: OverflowTag, S> {
    fn with_tagged<T1: Tag>(self, text: &'static str, tag: T1) -> Text<S::WithSegment, O>
    where
        S: SegmentStore<T1, T2>;

    fn with<T1: Tag + Default>(self, text: &'static str) -> Text<S::WithSegment, O>
    where
        S: SegmentStore<T1, T2>;
}

impl<T2: Tag, S, O: OverflowTag> WithTagged<T2, O, S> for Text<S, O> {
    fn with_tagged<T1: Tag>(self, text: &'static str, tag: T1) -> Text<S::WithSegment, O>
    where
        S: SegmentStore<T1, T2>,
    {
        Text {
            segments: self.segments.with_segment(text, tag),
            overflow_behavior: self.overflow_behavior,
        }
    }

    fn with<T1: Tag + Default>(self, text: &'static str) -> Text<S::WithSegment, O>
    where
        S: SegmentStore<T1, T2>,
    {
        Text {
            segments: self.segments.with_segment(text, Default::default()),
            overflow_behavior: self.overflow_behavior,
        }
    }
}

impl<S, O: OverflowTag> Text<S, O> {
    /// Set the overflow behavior explicitly.
    ///
    /// This is the "root" builder; all the other helpers forward into this.
    pub fn with_overflow<NO: OverflowTag>(self, overflow: Overflow<NO>) -> Text<S, NO> {
        Text {
            segments: self.segments,
            overflow_behavior: overflow,
        }
    }

    /// Convenience: disable wrapping and silently clip overflowing text.
    pub fn clip(self) -> Self {
        self.with_overflow(Overflow::Clip)
    }

    /// Convenience: wrap strictly on grapheme boundaries, never losing text.
    pub fn grapheme_wrap(self) -> Self {
        self.with_overflow(Overflow::GraphemeWrap)
    }

    /// Convenience: wrap on word boundaries when possible.
    pub fn word_wrap(self) -> Self {
        self.with_overflow(Overflow::WordWrap)
    }
}

pub trait EllipsisMethods<T2: Tag, S> {
    /// Convenience: render an ellipsis at the right edge on overflow.
    fn ellipsis<T1: Tag>(self) -> Text<S, NoOverflowTag<T1>>
    where
        S: SegmentStore<T1, T2>;

    /// Convenience: render an ellipsis at the right edge on overflow.
    fn ellipsis_tagged<T1: Tag + Clone>(self, tag: T1) -> Text<S, SomeOverflowTag<T1>>
    where
        S: SegmentStore<T1, T2>;

    /// Convenience: choose where the ellipsis goes (left, center, right).
    fn ellipsis_at<T1: Tag>(self, position: EllipsisPosition) -> Text<S, NoOverflowTag<T1>>
    where
        S: SegmentStore<T1, T2>;

    /// Convenience: choose where the ellipsis goes (left, center, right).
    fn ellipsis_tagged_at<T1: Tag + Clone>(
        self,
        position: EllipsisPosition,
        tag: T1,
    ) -> Text<S, SomeOverflowTag<T1>>
    where
        S: SegmentStore<T1, T2>;
}

impl<T2: Tag, S, O: OverflowTag> EllipsisMethods<T2, S> for Text<S, O> {
    fn ellipsis<T: Tag>(self) -> Text<S, NoOverflowTag<T>>
    where
        S: SegmentStore<T, T2>,
    {
        self.ellipsis_at(EllipsisPosition::Right)
    }

    fn ellipsis_tagged<T: Tag + Clone>(self, tag: T) -> Text<S, SomeOverflowTag<T>>
    where
        S: SegmentStore<T, T2>,
    {
        self.ellipsis_tagged_at(EllipsisPosition::Right, tag)
    }

    fn ellipsis_at<T: Tag>(self, position: EllipsisPosition) -> Text<S, NoOverflowTag<T>>
    where
        S: SegmentStore<T, T2>,
    {
        self.with_overflow(Overflow::Ellipsis(position, NoOverflowTag::default()))
    }

    fn ellipsis_tagged_at<T: Tag + Clone>(
        self,
        position: EllipsisPosition,
        tag: T,
    ) -> Text<S, SomeOverflowTag<T>>
    where
        S: SegmentStore<T, T2>,
    {
        self.with_overflow(Overflow::Ellipsis(position, SomeOverflowTag(tag)))
    }
}
