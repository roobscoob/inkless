use crate::{
    buffer::RenderBuffer,
    canvas::{NoDefaultTag, RenderBufferCanvas},
    render_position::RenderPosition,
    tag::Tag,
};

pub trait RenderBufferCanvasExt<T: Tag> {
    fn canvas_at<'a>(
        &'a mut self,
        position: RenderPosition,
    ) -> RenderBufferCanvas<'a, T, NoDefaultTag>;
}

impl<T: Tag, B: RenderBuffer<T>> RenderBufferCanvasExt<T> for B {
    fn canvas_at<'a>(
        &'a mut self,
        position: RenderPosition,
    ) -> RenderBufferCanvas<'a, T, NoDefaultTag> {
        RenderBufferCanvas {
            buffer: self,
            start_position: position,
            position,
            default_tag: NoDefaultTag,
        }
    }
}
