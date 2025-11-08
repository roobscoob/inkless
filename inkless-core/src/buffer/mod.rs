pub mod r#static;

use crate::{
    canvas::NoDefaultTag,
    grapheme::gph,
    render_position::RenderPosition,
    renderable::Renderable,
    tag::{Tag, sink::TagSink},
};

pub trait RenderBuffer<T: Tag> {
    /// Checks to see if setting this grapheme at this position would be within horizontal bounds
    ///
    /// Returns: `true` if the character would be entirely within the horizontal bounds of the buffer
    ///
    /// Returns: `false` if the character would horizontally overflow
    fn can_set_cell(&self, position: RenderPosition, c: &gph) -> bool;

    /// Sets the character and tag at this position.
    ///
    /// Returns: `true` ((and mutates the buffer) if the character is entirely within the horizontal bounds of the buffer
    ///
    /// Returns: `false` (and does not mutate the buffer) if the character horizontally overflowed
    fn set_tagged_cell(&mut self, position: RenderPosition, c: &gph, tag: T) -> bool;

    /// Sets the character at this position.
    ///
    /// Returns: `true` ((and mutates the buffer) if the character is entirely within the horizontal bounds of the buffer
    ///
    /// Returns: `false` (and does not mutate the buffer) if the character horizontally overflowed
    fn set_untagged_cell(&mut self, position: RenderPosition, c: &gph) -> bool;

    /// Returns the width of the RenderBuffer (if it has one). This is in the same units as `Grapheme::width` and `Grapheme::width_cjk`
    fn width(&self) -> Option<usize>;
}

pub trait RenderDispatcher<T: Tag> {
    fn render<R: Renderable<T, NoDefaultTag>, S: TagSink<T>>(
        sink: impl Into<S>,
        renderable: R,
        width: usize,
    ) -> S::Result;
}
