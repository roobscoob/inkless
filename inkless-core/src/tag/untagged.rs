use crate::tag::Tag;

#[derive(Clone, Copy, Debug, Default)]
pub struct Untagged;

impl Tag for Untagged {}
