use core::marker::PhantomData;

use crate::{
    canvas::Canvas,
    renderable::Renderable,
    tag::Tag,
    theme::{Theme, canvas::ThemeCanvas},
};

#[repr(transparent)]
pub struct ThemedRenderable<Ot: Tag, Th: Theme<Ot>, R: ?Sized + Renderable<Ot>>(
    PhantomData<(Ot, Th)>,
    R,
);

pub trait RenderableThemeExt<Ot: Tag>: Renderable<Ot> {
    fn as_theme<'a, Th: Theme<Ot>>(&'a self) -> &'a ThemedRenderable<Ot, Th, Self>
    where
        Self: Renderable<Ot>;

    fn with_theme<'a, Th: Theme<Ot>>(self) -> ThemedRenderable<Ot, Th, Self>
    where
        Self: Renderable<Ot>;
}

impl<T: Tag, R: Renderable<T>> RenderableThemeExt<T> for R {
    fn as_theme<'a, Th: Theme<T>>(&'a self) -> &'a ThemedRenderable<T, Th, Self>
    where
        Self: Renderable<T>,
    {
        // SAFETY: `ThemedRenderable<T, Th, Self>` is #[repr(transparent)] and its only
        // non-ZST field is `Self`, so `&Self` and `&ThemedRenderable<T, Th, Self>`
        // have the same layout and metadata. Additionally, the type requires
        // `Self: Renderable<ThemedTag<T, Th>>`, which is enforced by this `where` clause.
        unsafe { &*(self as *const Self as *const ThemedRenderable<T, Th, Self>) }
    }

    fn with_theme<'a, Th: Theme<T>>(self) -> ThemedRenderable<T, Th, Self>
    where
        Self: Renderable<T>,
    {
        ThemedRenderable(Default::default(), self)
    }
}

impl<Tt: Tag, Ot: Tag, Th: Theme<Ot, Result = Tt>, R> Renderable<Tt> for ThemedRenderable<Ot, Th, R>
where
    R: Renderable<Ot>,
{
    fn render_into<'buffer_reference>(
        &self,
        canvas: &mut dyn Canvas<Tt>,
    ) -> Result<(), crate::renderable::RenderableError> {
        let mut c = ThemeCanvas::<Tt, Th> {
            inner: canvas,
            theme: PhantomData::default(),
        };

        self.1.render_into(&mut c)
    }
}
