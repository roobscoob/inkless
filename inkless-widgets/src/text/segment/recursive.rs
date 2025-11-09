use inkless_core::{renderable::Renderable, tag::Tag};

use crate::text::segment::{SegmentStore, TextSegment};

#[derive(Default)]
pub struct RecursiveSegmentStoreNone;

pub struct RecursiveSegmentStoreSomeRenderable<Tail, R> {
    pub renderable: R,
    pub tail: Tail,
}

pub struct RecursiveSegmentStoreSomeSegment<T, Tail> {
    pub segment: &'static str,
    pub tag: T,
    pub tail: Tail,
}

impl<T1: Tag, T2: Tag> SegmentStore<T1, T2> for RecursiveSegmentStoreNone {
    type WithRenderable<R: Renderable<T2>> = RecursiveSegmentStoreSomeRenderable<Self, R>;
    type WithSegment = RecursiveSegmentStoreSomeSegment<T1, Self>;

    fn len(&self) -> usize {
        0
    }

    fn get<'a>(&'a self, _: usize) -> Option<super::TextSegment<'a, T1, T2>> {
        None
    }

    fn with_renderable<R: Renderable<T2>>(self, value: R) -> Self::WithRenderable<R> {
        RecursiveSegmentStoreSomeRenderable {
            renderable: value,
            tail: self,
        }
    }

    fn with_segment(self, text: &'static str, tag: T1) -> Self::WithSegment {
        RecursiveSegmentStoreSomeSegment {
            segment: text,
            tag,
            tail: self,
        }
    }
}

impl<T1: Tag, T2: Tag, Tail: SegmentStore<T1, T2>, R: Renderable<T2>> SegmentStore<T1, T2>
    for RecursiveSegmentStoreSomeRenderable<Tail, R>
{
    type WithRenderable<R2: Renderable<T2>> = RecursiveSegmentStoreSomeRenderable<Self, R2>;
    type WithSegment = RecursiveSegmentStoreSomeSegment<T1, Self>;

    fn len(&self) -> usize {
        1 + self.tail.len()
    }

    fn get<'a>(&'a self, index: usize) -> Option<super::TextSegment<'a, T1, T2>> {
        (index == self.tail.len())
            .then_some(Some(TextSegment::Renderable(&self.renderable)))
            .unwrap_or_else(|| self.tail.get(index))
    }

    fn with_renderable<R2: Renderable<T2>>(self, value: R2) -> Self::WithRenderable<R2> {
        RecursiveSegmentStoreSomeRenderable {
            renderable: value,
            tail: self,
        }
    }

    fn with_segment(self, text: &'static str, tag: T1) -> Self::WithSegment {
        RecursiveSegmentStoreSomeSegment {
            segment: text,
            tag,
            tail: self,
        }
    }
}

impl<T1: Tag, T2: Tag, Tail: SegmentStore<T1, T2>> SegmentStore<T1, T2>
    for RecursiveSegmentStoreSomeSegment<T1, Tail>
{
    type WithRenderable<R2: Renderable<T2>> = RecursiveSegmentStoreSomeRenderable<Self, R2>;
    type WithSegment = RecursiveSegmentStoreSomeSegment<T1, Self>;

    fn len(&self) -> usize {
        1 + self.tail.len()
    }

    fn get<'a>(&'a self, index: usize) -> Option<super::TextSegment<'a, T1, T2>> {
        (index == self.tail.len())
            .then_some(Some(TextSegment::Segment(self.segment, &self.tag)))
            .unwrap_or_else(|| self.tail.get(index))
    }

    fn with_renderable<R2: Renderable<T2>>(self, value: R2) -> Self::WithRenderable<R2> {
        RecursiveSegmentStoreSomeRenderable {
            renderable: value,
            tail: self,
        }
    }

    fn with_segment(self, text: &'static str, tag: T1) -> Self::WithSegment {
        RecursiveSegmentStoreSomeSegment {
            segment: text,
            tag,
            tail: self,
        }
    }
}
