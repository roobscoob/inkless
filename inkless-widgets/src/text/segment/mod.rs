pub mod recursive;

use inkless_core::{renderable::Renderable, tag::Tag};

pub enum TextSegment<'l, T1: Tag, T2: Tag> {
    Renderable(&'l dyn Renderable<T2>),
    Segment(&'static str, &'l T1),
}

pub trait SegmentStore<T1: Tag, T2: Tag> {
    type WithRenderable<R: Renderable<T2>>: SegmentStore<T1, T2>;
    type WithSegment: SegmentStore<T1, T2>;

    fn len(&self) -> usize;
    fn get<'a>(&'a self, index: usize) -> Option<TextSegment<'a, T1, T2>>;
    fn with_renderable<R: Renderable<T2>>(self, value: R) -> Self::WithRenderable<R>;
    fn with_segment(self, text: &'static str, tag: T1) -> Self::WithSegment;
}

// impl<'l, T: Tag> Debug for TextSegment<'l, T> {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         match self {
//             Self::Segment(string, _) => {
//                 f.write_fmt(format_args!("[{}] {:?}", type_name::<T>(), string))
//             }
//             Self::Renderable(_) => f.write_fmt(format_args!("{{?}}")),
//         }
//     }
// }
