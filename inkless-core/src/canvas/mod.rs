pub mod ext;
pub mod summary;

use crate::{
    buffer::RenderBuffer,
    canvas::summary::RenderBufferCanvasSummary,
    grapheme::{char::CharGrapheme, gph},
    render_position::RenderPosition,
    renderable::Renderable,
    tag::Tag,
};

pub trait DefaultTag<T: Tag>: Clone {
    fn get(&self) -> Option<T>;
}

#[derive(Clone)]
pub struct NoDefaultTag;

#[derive(Clone)]
pub struct ClonableTag<T: Tag + Clone>(T);

impl<T: Tag> DefaultTag<T> for NoDefaultTag {
    fn get(&self) -> Option<T> {
        None
    }
}

impl<T: Tag + Clone> DefaultTag<T> for ClonableTag<T> {
    fn get(&self) -> Option<T> {
        Some(self.0.clone())
    }
}

pub struct RenderBufferCanvas<'reference, T: Tag, D: DefaultTag<T>> {
    buffer: &'reference mut (dyn RenderBuffer<T> + 'reference),
    start_position: RenderPosition,
    position: RenderPosition,
    default_tag: D,
}

impl<'reference, T: Tag, D: DefaultTag<T>> RenderBufferCanvas<'reference, T, D> {
    /// Writes the renderable to the current position, using the default_tag of the current canvas.
    ///
    /// Returns: `RenderBufferSummary` describing the start and end position.
    pub fn write<'a>(
        &'a mut self,
        renderable: &dyn Renderable<T, D>,
    ) -> Result<RenderBufferCanvasSummary, ()> {
        let mut new_canvas: RenderBufferCanvas<'a, T, D> = RenderBufferCanvas {
            buffer: &mut *self.buffer,
            position: self.position,
            start_position: self.position,
            default_tag: self.default_tag.clone(),
        };

        renderable.render_into(&mut new_canvas)?;

        let result = new_canvas.end();

        self.position = result.end_position;

        Ok(result)
    }

    /// Writes the renderable to the current position, using the provided tag as the default_tag.
    ///
    /// Returns: `RenderBufferSummary` describing the start and end position.
    pub fn write_tagged(
        &mut self,
        renderable: &dyn Renderable<T, ClonableTag<T>>,
        default_tag: impl Into<T>,
    ) -> Result<RenderBufferCanvasSummary, ()>
    where
        T: Clone,
    {
        let mut new_canvas = RenderBufferCanvas {
            buffer: self.buffer,
            position: self.position,
            start_position: self.position,
            default_tag: ClonableTag(default_tag.into()),
        };

        renderable.render_into(&mut new_canvas)?;

        let result = new_canvas.end();

        self.position = result.end_position;

        Ok(result)
    }

    /// Sets the character at the canvas head using the canvas's default tag.
    ///
    /// Returns: `true` (and mutates the canvas) if the character is entirely within the horizontal bounds of the buffer
    ///
    /// Returns: `false` (and does not mutate the buffer) if the character horizontally overflowed
    pub fn set_char(&mut self, char: char) -> bool {
        self.set_gph(&CharGrapheme::from(char))
    }

    /// Sets the grapheme at the canvas head using the canvas's default tag.
    ///
    /// Returns: `true` (and mutates the canvas) if the grapheme is entirely within the horizontal bounds of the buffer
    ///
    /// Returns: `false` (and does not mutate the buffer) if the grapheme horizontally overflowed
    pub fn set_gph(&mut self, gph: &gph) -> bool {
        let result = if let Some(tag) = self.default_tag.get() {
            self.buffer.set_tagged_cell(self.position, gph, tag)
        } else {
            self.buffer.set_untagged_cell(self.position, gph)
        };

        if result {
            self.cursor_right();
        }

        result
    }

    /// Sets the character at the canvas head using the provided tag.
    ///
    /// Returns: `true` (and mutates the canvas) if the character is entirely within the horizontal bounds of the buffer
    ///
    /// Returns: `false` (and does not mutate the buffer) if the character horizontally overflowed
    pub fn set_tagged_char(&mut self, char: char, tag: impl Into<T>) -> bool {
        self.set_tagged_gph(&CharGrapheme::from(char), tag.into())
    }

    /// Sets the grapheme at the canvas head using the provided tag.
    ///
    /// Returns: `true` (and mutates the canvas) if the grapheme is entirely within the horizontal bounds of the buffer
    ///
    /// Returns: `false` (and does not mutate the buffer) if the grapheme horizontally overflowed
    pub fn set_tagged_gph(&mut self, gph: &gph, tag: impl Into<T>) -> bool {
        let result = self.buffer.set_tagged_cell(self.position, gph, tag.into());

        if result {
            self.cursor_right();
        }

        result
    }

    /// Finishes the canvas returning a summary
    fn end(self) -> RenderBufferCanvasSummary {
        RenderBufferCanvasSummary {
            end_position: self.position,
            start_position: self.start_position,
        }
    }

    pub fn get_start_position(&self) -> RenderPosition {
        self.start_position
    }

    pub fn get_position(&self) -> RenderPosition {
        self.position
    }

    pub fn set_position(&mut self, position: RenderPosition) -> &mut Self {
        self.position = position;
        self
    }

    pub fn set_column(&mut self, column: usize) -> &mut Self {
        self.position = RenderPosition::new(self.position.line(), column);
        self
    }

    pub fn set_line(&mut self, line: usize) -> &mut Self {
        self.position = RenderPosition::new(line, self.position.column());
        self
    }

    pub fn cursor_down(&mut self) -> &mut Self {
        self.position = self.position.down(1);
        self
    }

    pub fn cursor_down_by(&mut self, count: usize) -> &mut Self {
        self.position = self.position.down(count);
        self
    }

    pub fn try_cursor_up(&mut self) -> bool {
        if let Some(new_position) = self.position.try_up(1) {
            self.position = new_position;
            true
        } else {
            false
        }
    }

    pub fn try_cursor_up_by(&mut self, count: usize) -> bool {
        if let Some(new_position) = self.position.try_up(count) {
            self.position = new_position;
            true
        } else {
            false
        }
    }

    pub fn cursor_up(&mut self) -> &mut Self {
        self.position = self
            .position
            .try_up(1)
            .expect("Expected to be able to move the cursor up");
        self
    }

    pub fn cursor_up_by(&mut self, count: usize) -> &mut Self {
        self.position = self
            .position
            .try_up(count)
            .expect("Expected to be able to move the cursor up");
        self
    }

    pub fn try_cursor_left(&mut self) -> bool {
        if let Some(new_position) = self.position.try_left(1) {
            self.position = new_position;
            true
        } else {
            false
        }
    }

    pub fn try_cursor_left_by(&mut self, count: usize) -> bool {
        if let Some(new_position) = self.position.try_left(count) {
            self.position = new_position;
            true
        } else {
            false
        }
    }

    pub fn cursor_left(&mut self) -> &mut Self {
        self.position = self
            .position
            .try_left(1)
            .expect("Expected to be able to move the cursor left");
        self
    }

    pub fn cursor_left_by(&mut self, count: usize) -> &mut Self {
        self.position = self
            .position
            .try_left(count)
            .expect("Expected to be able to move the cursor left");
        self
    }

    pub fn cursor_right(&mut self) -> &mut Self {
        self.position = self.position.right(1);
        self
    }

    pub fn cursor_right_by(&mut self, count: usize) -> &mut Self {
        self.position = self.position.right(count);
        self
    }
}
