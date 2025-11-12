pub mod canvas;
pub mod ext;
pub mod clean;

use crate::tag::Tag;

pub trait Theme<T: Tag> {
    type Result: Tag;

    fn translate(from: T) -> Self::Result;
}
