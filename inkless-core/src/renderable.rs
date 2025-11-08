use crate::{
    canvas::{DefaultTag, RenderBufferCanvas},
    tag::Tag,
};

pub trait Renderable<T: Tag, D: DefaultTag<T>> {
    fn render_into<'buffer_reference>(
        &self,
        canvas: &mut RenderBufferCanvas<'buffer_reference, T, D>,
    ) -> Result<(), ()>;
}

impl<'a, T: Tag, D: DefaultTag<T>, O: Renderable<T, D>> Renderable<T, D> for &'a O {
    fn render_into<'buffer_reference>(
        &self,
        canvas: &mut RenderBufferCanvas<'buffer_reference, T, D>,
    ) -> Result<(), ()> {
        (**self).render_into(canvas)
    }
}
