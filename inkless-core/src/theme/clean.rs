use crate::{
    tag::{Tag, untagged::Untagged},
    theme::Theme,
};

pub struct Clean;

impl<T: Tag> Theme<T> for Clean {
    type Result = Untagged;

    fn translate(_: T) -> Self::Result {
        Untagged
    }
}
