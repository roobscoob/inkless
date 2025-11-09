use crate::{
    buffer::RenderBuffer, canvas::RenderBufferCanvas, render_position::RenderPosition, tag::Tag,
};

pub trait RenderBufferCanvasExt<T: Tag> {
    fn canvas_at<'a>(&'a mut self, position: RenderPosition) -> RenderBufferCanvas<'a, T>;
}

impl<T: Tag, B: RenderBuffer<T>> RenderBufferCanvasExt<T> for B {
    fn canvas_at<'a>(&'a mut self, position: RenderPosition) -> RenderBufferCanvas<'a, T> {
        RenderBufferCanvas {
            buffer: self,
            start_position: position,
            position,
        }
    }
}
