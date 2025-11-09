use core::marker::PhantomData;

use inkless_core::{renderable::Renderable, tag::Tag};

use crate::text::segment::{SegmentStore, SegmentStoreFetch, TextSegment};

#[derive(Default)]
pub struct RecursiveSegmentStoreNone<T2: Tag>(pub(crate) PhantomData<fn(T2) -> ()>);

pub struct RecursiveSegmentStoreSomeRenderable<Tail, T2: Tag, R: Renderable<T2>> {
    pub(crate) renderable: R,
    pub(crate) tail: Tail,
    pub(crate) _ph: PhantomData<fn(T2) -> ()>,
}

pub struct RecursiveSegmentStoreSomeSegment<T1, T2, Tail> {
    pub(crate) segment: &'static str,
    pub(crate) tag: T1,
    pub(crate) tail: Tail,
    pub(crate) _ph: PhantomData<fn(T2) -> ()>,
}

impl<T1: Tag, T2: Tag> SegmentStore<T1> for RecursiveSegmentStoreNone<T2> {
    type T2 = T2;

    type WithRenderable<R: Renderable<Self::T2>> =
        RecursiveSegmentStoreSomeRenderable<Self, Self::T2, R>;

    type WithSegment = RecursiveSegmentStoreSomeSegment<T1, T2, Self>;

    fn len(&self) -> usize {
        0
    }

    fn with_renderable<R: Renderable<Self::T2>>(self, value: R) -> Self::WithRenderable<R> {
        RecursiveSegmentStoreSomeRenderable {
            renderable: value,
            tail: self,
            _ph: PhantomData::default(),
        }
    }

    fn with_segment(self, text: &'static str, tag: T1) -> Self::WithSegment {
        RecursiveSegmentStoreSomeSegment {
            segment: text,
            tag,
            tail: self,
            _ph: PhantomData::default(),
        }
    }
}

impl<T1: Tag, T2: Tag, T3: Tag> SegmentStoreFetch<T1, T3> for RecursiveSegmentStoreNone<T2> {
    fn get<'a>(&'a self, _: usize) -> Option<super::TextSegment<'a, T1, T3>> {
        None
    }
}

impl<T1: Tag, T2: Tag, Tail: SegmentStore<T1, T2 = T2>, R: Renderable<T2>> SegmentStore<T1>
    for RecursiveSegmentStoreSomeRenderable<Tail, T2, R>
{
    type T2 = T2;

    type WithRenderable<R2: Renderable<Self::T2>> =
        RecursiveSegmentStoreSomeRenderable<Self, Self::T2, R2>;

    type WithSegment = RecursiveSegmentStoreSomeSegment<T1, T2, Self>;

    fn len(&self) -> usize {
        1 + self.tail.len()
    }

    fn with_renderable<R2: Renderable<Self::T2>>(self, value: R2) -> Self::WithRenderable<R2> {
        RecursiveSegmentStoreSomeRenderable {
            renderable: value,
            tail: self,
            _ph: PhantomData::default(),
        }
    }

    fn with_segment(self, text: &'static str, tag: T1) -> Self::WithSegment {
        RecursiveSegmentStoreSomeSegment {
            segment: text,
            tag,
            tail: self,
            _ph: PhantomData::default(),
        }
    }
}

impl<
    T1: Tag,
    T2: Tag,
    T3: Tag,
    Tail: SegmentStore<T1, T2 = T2> + SegmentStoreFetch<T1, T3>,
    R: Renderable<T2> + Renderable<T3>,
> SegmentStoreFetch<T1, T3> for RecursiveSegmentStoreSomeRenderable<Tail, T2, R>
{
    fn get<'a>(&'a self, index: usize) -> Option<super::TextSegment<'a, T1, T3>> {
        (index == self.tail.len())
            .then_some(Some(TextSegment::Renderable(&self.renderable)))
            .unwrap_or_else(|| self.tail.get(index))
    }
}

impl<T1: Tag, T2: Tag, Tail: SegmentStore<T1, T2 = T2>> SegmentStore<T1>
    for RecursiveSegmentStoreSomeSegment<T1, T2, Tail>
{
    type T2 = T2;

    type WithRenderable<R2: Renderable<T2>> = RecursiveSegmentStoreSomeRenderable<Self, T2, R2>;

    type WithSegment = RecursiveSegmentStoreSomeSegment<T1, T2, Self>;

    fn len(&self) -> usize {
        1 + self.tail.len()
    }

    fn with_renderable<R2: Renderable<Self::T2>>(self, value: R2) -> Self::WithRenderable<R2> {
        RecursiveSegmentStoreSomeRenderable {
            renderable: value,
            tail: self,
            _ph: PhantomData::default(),
        }
    }

    fn with_segment(self, text: &'static str, tag: T1) -> Self::WithSegment {
        RecursiveSegmentStoreSomeSegment {
            segment: text,
            tag,
            tail: self,
            _ph: PhantomData::default(),
        }
    }
}

impl<T1: Tag, T2: Tag, T3: Tag, Tail: SegmentStore<T1, T2 = T2> + SegmentStoreFetch<T1, T3>>
    SegmentStoreFetch<T1, T3> for RecursiveSegmentStoreSomeSegment<T1, T2, Tail>
{
    fn get<'a>(&'a self, index: usize) -> Option<super::TextSegment<'a, T1, T3>> {
        (index == self.tail.len())
            .then_some(Some(TextSegment::Segment(self.segment, &self.tag)))
            .unwrap_or_else(|| self.tail.get(index))
    }
}
