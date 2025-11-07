use crate::canvas::RenderBufferCanvas;

pub trait Renderable<'tag> {
    fn render_into<'buffer_reference>(
        &self,
        canvas: &mut RenderBufferCanvas<'tag, 'buffer_reference>,
    ) -> Result<(), ()>;
}

impl<'a, 't, O: Renderable<'t>> Renderable<'t> for &'a O {
    fn render_into<'buffer_reference>(
        &self,
        canvas: &mut RenderBufferCanvas<'t, 'buffer_reference>,
    ) -> Result<(), ()> {
        (**self).render_into(canvas)
    }
}
