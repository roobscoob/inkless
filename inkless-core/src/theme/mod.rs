pub mod ext;

use core::{marker::PhantomData, ops::Deref};

use crate::{
    canvas::RenderBufferCanvas,
    renderable::{Renderable, RenderableError},
    tag::Tag,
};

pub trait Theme<T: Tag> {
    type Result: Tag;

    fn translate(from: T) -> Self::Result;
}

#[repr(transparent)]
pub struct ThemedTag<Ot: Tag, Th: Theme<Ot>>(pub Th::Result, PhantomData<(Ot, Th)>);

impl<Ot, Th> ThemedTag<Ot, Th>
where
    Ot: Tag,
    Th: Theme<Ot>,
{
    pub fn from_result(result: Th::Result) -> Self {
        Self(result, Default::default())
    }
}

impl<Ot, Th> Tag for ThemedTag<Ot, Th>
where
    Ot: Tag,
    Th: Theme<Ot>,
{
}

impl<Ot, Th> From<Ot> for ThemedTag<Ot, Th>
where
    Ot: Tag,
    Th: Theme<Ot>,
{
    fn from(src: Ot) -> Self {
        ThemedTag(Th::translate(src), PhantomData)
    }
}

impl<Ot: Tag, Th: Theme<Ot>> Deref for ThemedTag<Ot, Th> {
    type Target = Th::Result;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Ot: Tag, Th: Theme<Ot>> Default for ThemedTag<Ot, Th>
where
    Th::Result: Default,
{
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
