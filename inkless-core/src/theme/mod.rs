pub mod canvas;
pub mod ext;

use crate::tag::Tag;

pub trait Theme<T: Tag> {
    type Result: Tag;

    fn translate(from: T) -> Self::Result;
}
