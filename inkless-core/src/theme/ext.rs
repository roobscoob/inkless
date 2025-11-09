use crate::{
    renderable::Renderable,
    tag::Tag,
    theme::{Theme, ThemedTag},
};

pub trait RenderableThemeExt<Ot: Tag>: Sized + Renderable<Ot> {
    fn theme<Th: Theme<Ot>>(self) -> impl Renderable<ThemedTag<Ot, Th>>
    where
        Self: Renderable<ThemedTag<Ot, Th>>;
}

impl<T: Tag, R: Renderable<T>> RenderableThemeExt<T> for R {
    fn theme<Th: Theme<T>>(self) -> impl Renderable<ThemedTag<T, Th>>
    where
        Self: Renderable<ThemedTag<T, Th>>,
    {
        self
    }
}
