pub mod buffer;
pub mod ext;
pub mod summary;

use crate::{
    canvas::summary::CanvasSummary,
    grapheme::gph,
    render_position::RenderPosition,
    renderable::{Renderable, RenderableError},
    tag::Tag,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AmbiguityPolicy {
    Standard,
    Wide,
}

pub trait Canvas<T: Tag> {
    /// Writes the renderable to the current position.
    ///
    /// Returns: `CanvasSummary` describing the start and end position.
    fn write<'a>(
        &'a mut self,
        renderable: &dyn Renderable<T>,
    ) -> Result<CanvasSummary, RenderableError>;

    /// Sets the character at the canvas head using the provided tag.
    ///
    /// Returns: `true` (and mutates the canvas) if the character is entirely within the horizontal bounds of the buffer
    ///
    /// Returns: `false` (and does not mutate the buffer) if the character horizontally overflowed
    fn set_char(&mut self, ch: char, tag: T) -> bool;

    /// Sets the grapheme at the canvas head using the provided tag.
    ///
    /// Returns: `true` (and mutates the canvas) if the grapheme is entirely within the horizontal bounds of the buffer
    ///
    /// Returns: `false` (and does not mutate the buffer) if the grapheme horizontally overflowed
    fn set_gph(&mut self, v: &gph, tag: T) -> bool;

    /// Checks to see if the grapheme at the canvas head can be printed.
    ///
    /// Returns: `true` (and mutates the cursor) if the grapheme is entirely within the horizontal bounds of the buffer
    ///
    /// Returns: `false` (and does not mutate the cursor) if the grapheme horizontally overflowed
    fn can_set_gph(&mut self, v: &gph) -> bool;

    fn get_start_position(&self) -> RenderPosition;

    fn get_position(&self) -> RenderPosition;

    fn set_position(&mut self, position: RenderPosition) -> &mut dyn Canvas<T>;

    fn set_column(&mut self, column: usize) -> &mut dyn Canvas<T>;

    fn set_line(&mut self, line: usize) -> &mut dyn Canvas<T>;

    fn cursor_down(&mut self) -> &mut dyn Canvas<T>;

    fn cursor_down_by(&mut self, count: usize) -> &mut dyn Canvas<T>;

    fn try_cursor_up(&mut self) -> bool;

    fn try_cursor_up_by(&mut self, count: usize) -> bool;

    fn cursor_up(&mut self) -> &mut dyn Canvas<T>;

    fn cursor_up_by(&mut self, count: usize) -> &mut dyn Canvas<T>;

    fn try_cursor_left(&mut self) -> bool;

    fn try_cursor_left_by(&mut self, count: usize) -> bool;

    fn cursor_left(&mut self) -> &mut dyn Canvas<T>;

    fn cursor_left_by(&mut self, count: usize) -> &mut dyn Canvas<T>;

    fn cursor_right(&mut self) -> &mut dyn Canvas<T>;

    fn cursor_right_by(&mut self, count: usize) -> &mut dyn Canvas<T>;
}
