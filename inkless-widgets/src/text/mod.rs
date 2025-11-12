use core::marker::PhantomData;

use inkless_core::{
    renderable::Renderable,
    tag::{Tag, untagged::Untagged},
};

use crate::text::{
    overflow::{EllipsisPosition, Overflow},
    segment::{
        SegmentStore,
        recursive::{RecursiveSegmentStoreNone, RecursiveSegmentStoreSomeSegment},
    },
};

pub mod overflow;
pub mod renderable;
pub mod segment;
pub mod tag;

pub struct Text<S, T: Tag> {
    segments: S,
    overflow_behavior: Overflow,
    _ph: PhantomData<T>,
}

impl<S, T: Tag> Text<S, T> {
    fn from_store(store: S) -> Self {
        Self {
            segments: store,
            overflow_behavior: Default::default(),
            _ph: PhantomData::default(),
        }
    }
}

impl<T2: Tag> Text<RecursiveSegmentStoreNone<T2>, Untagged> {
    pub fn empty<T: Tag>() -> Text<RecursiveSegmentStoreNone<T2>, T> {
        Text::from_store(RecursiveSegmentStoreNone(Default::default()))
    }

    pub fn of_tagged<T: Tag>(
        text: &'static str,
        tag: T,
    ) -> Text<RecursiveSegmentStoreSomeSegment<T, T2, RecursiveSegmentStoreNone<T2>>, T> {
        Text {
            segments: RecursiveSegmentStoreSomeSegment {
                segment: text,
                tag,
                tail: RecursiveSegmentStoreNone(Default::default()),
                _ph: Default::default(),
            },
            overflow_behavior: Overflow::default(),
            _ph: Default::default(),
        }
    }

    pub fn of<T: Tag>(
        text: &'static str,
    ) -> Text<RecursiveSegmentStoreSomeSegment<T, T2, RecursiveSegmentStoreNone<T2>>, T>
    where
        T: Default,
    {
        Text {
            segments: RecursiveSegmentStoreSomeSegment {
                segment: text,
                tag: Default::default(),
                tail: RecursiveSegmentStoreNone(Default::default()),
                _ph: Default::default(),
            },
            overflow_behavior: Overflow::default(),
            _ph: Default::default(),
        }
    }
}

impl<T: Tag + Default, T2: Tag> From<&'static str>
    for Text<RecursiveSegmentStoreSomeSegment<T, T2, RecursiveSegmentStoreNone<T2>>, T>
{
    fn from(value: &'static str) -> Self {
        Self::from_store(RecursiveSegmentStoreSomeSegment {
            segment: value,
            tag: T::default(),
            tail: RecursiveSegmentStoreNone(Default::default()),
            _ph: Default::default(),
        })
    }
}

pub trait TextWithRenderable<Ta: Tag, C, S> {
    fn with_component<Tb: Tag>(self, value: C) -> Text<S::WithRenderable<C>, Ta>
    where
        S: SegmentStore<Ta, T2 = Tb>,
        C: Renderable<Tb>;
}

impl<Ta: Tag, C, S> TextWithRenderable<Ta, C, S> for Text<S, Ta> {
    fn with_component<Tb: Tag>(self, value: C) -> Text<S::WithRenderable<C>, Ta>
    where
        S: SegmentStore<Ta, T2 = Tb>,
        C: Renderable<Tb>,
    {
        Text {
            segments: self.segments.with_renderable(value),
            overflow_behavior: self.overflow_behavior,
            _ph: PhantomData::default(),
        }
    }
}

pub trait WithTagged<T2: Tag, S> {
    fn with_tagged<T1: Tag>(self, text: &'static str, tag: T1) -> Text<S::WithSegment, T1>
    where
        S: SegmentStore<T1, T2 = T2>;

    fn with<T1: Tag + Default>(self, text: &'static str) -> Text<S::WithSegment, T1>
    where
        S: SegmentStore<T1, T2 = T2>;
}

impl<_T: Tag, T2: Tag, S> WithTagged<T2, S> for Text<S, _T> {
    fn with_tagged<T1: Tag>(self, text: &'static str, tag: T1) -> Text<S::WithSegment, T1>
    where
        S: SegmentStore<T1, T2 = T2>,
    {
        Text {
            segments: self.segments.with_segment(text, tag),
            overflow_behavior: self.overflow_behavior,
            _ph: PhantomData::default(),
        }
    }

    fn with<T1: Tag + Default>(self, text: &'static str) -> Text<S::WithSegment, T1>
    where
        S: SegmentStore<T1, T2 = T2>,
    {
        Text {
            segments: self.segments.with_segment(text, Default::default()),
            overflow_behavior: self.overflow_behavior,
            _ph: PhantomData::default(),
        }
    }
}

impl<T1: Tag, S: SegmentStore<T1>> Text<S, T1> {
    /// Set the overflow behavior explicitly.
    ///
    /// This is the "root" builder; all the other helpers forward into this.
    pub fn with_overflow(self, overflow: Overflow) -> Self {
        Self {
            segments: self.segments,
            overflow_behavior: overflow,
            _ph: PhantomData::default(),
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

    /// Convenience: render an ellipsis at the right edge on overflow.
    pub fn ellipsis(self) -> Self {
        self.ellipsis_at(EllipsisPosition::Right)
    }

    /// Convenience: choose where the ellipsis goes (left, center, right).
    pub fn ellipsis_at(self, position: EllipsisPosition) -> Self {
        self.with_overflow(Overflow::Ellipsis(position))
    }
}
