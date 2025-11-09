pub mod sink;
pub mod untagged;

pub trait Tag {}

impl<T: Tag + ?Sized> Tag for &T {}

impl<T: Tag> Tag for Option<T> {}
