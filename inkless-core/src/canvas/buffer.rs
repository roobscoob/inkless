use crate::{
    buffer::RenderBuffer,
    canvas::{Canvas, summary::CanvasSummary},
    grapheme::{char::CharGrapheme, gph},
    render_position::RenderPosition,
    renderable::{Renderable, RenderableError},
    tag::Tag,
};

pub struct RenderBufferCanvas<'reference, T: Tag> {
    pub(super) buffer: &'reference mut (dyn RenderBuffer<T> + 'reference),
    pub(super) start_position: RenderPosition,
    pub(super) position: RenderPosition,
}

impl<'reference, T: Tag> Canvas<T> for RenderBufferCanvas<'reference, T> {
    /// Writes the renderable to the current position.
    ///
    /// Returns: `RenderBufferSummary` describing the start and end position.
    fn write<'a, Nt: Tag + Into<T>, C: Canvas<Nt>>(
        &'a mut self,
        renderable: &dyn Renderable<Nt, C>,
    ) -> Result<CanvasSummary, RenderableError> {
        let mut new_canvas: RenderBufferCanvas<'a, T> = RenderBufferCanvas {
            buffer: &mut *self.buffer,
            position: self.position,
            start_position: self.position,
        };

        renderable.render_into(&mut new_canvas)?;

        let result = new_canvas.end();

        self.position = result.end_position;

        Ok(result)
    }

    /// Sets the character at the canvas head using the provided tag.
    ///
    /// Returns: `true` (and mutates the canvas) if the character is entirely within the horizontal bounds of the buffer
    ///
    /// Returns: `false` (and does not mutate the buffer) if the character horizontally overflowed
    fn set_char(&mut self, char: char, tag: impl Into<T>) -> bool {
        self.set_gph(&CharGrapheme::from(char), tag.into())
    }

    /// Sets the grapheme at the canvas head using the provided tag.
    ///
    /// Returns: `true` (and mutates the canvas) if the grapheme is entirely within the horizontal bounds of the buffer
    ///
    /// Returns: `false` (and does not mutate the buffer) if the grapheme horizontally overflowed
    fn set_gph(&mut self, v: &gph, tag: impl Into<T>) -> bool {
        let result = self.buffer.set_cell(self.position, v, tag.into());

        if result {
            self.cursor_right_by(v.width(self.buffer.ambiguity_policy()));
        }

        result
    }

    /// Checks to see if the grapheme at the canvas head can be printed.
    ///
    /// Returns: `true` (and mutates the cursor) if the grapheme is entirely within the horizontal bounds of the buffer
    ///
    /// Returns: `false` (and does not mutate the cursor) if the grapheme horizontally overflowed
    fn can_set_gph(&mut self, v: &gph) -> bool {
        let result = self.buffer.can_set_cell(self.position, v);

        if result {
            self.cursor_right_by(v.width(self.buffer.ambiguity_policy()));
        }

        result
    }

    /// Finishes the canvas returning a summary
    fn end(self) -> CanvasSummary {
        CanvasSummary {
            end_position: self.position,
            start_position: self.start_position,
        }
    }

    fn get_start_position(&self) -> RenderPosition {
        self.start_position
    }

    fn get_position(&self) -> RenderPosition {
        self.position
    }

    fn set_position(&mut self, position: RenderPosition) -> &mut Self {
        self.position = position;
        self
    }

    fn set_column(&mut self, column: usize) -> &mut Self {
        self.position = RenderPosition::new(self.position.line(), column);
        self
    }

    fn set_line(&mut self, line: usize) -> &mut Self {
        self.position = RenderPosition::new(line, self.position.column());
        self
    }

    fn cursor_down(&mut self) -> &mut Self {
        self.position = self.position.down(1);
        self
    }

    fn cursor_down_by(&mut self, count: usize) -> &mut Self {
        self.position = self.position.down(count);
        self
    }

    fn try_cursor_up(&mut self) -> bool {
        if let Some(new_position) = self.position.try_up(1) {
            self.position = new_position;
            true
        } else {
            false
        }
    }

    fn try_cursor_up_by(&mut self, count: usize) -> bool {
        if let Some(new_position) = self.position.try_up(count) {
            self.position = new_position;
            true
        } else {
            false
        }
    }

    fn cursor_up(&mut self) -> &mut Self {
        self.position = self
            .position
            .try_up(1)
            .expect("Expected to be able to move the cursor up");
        self
    }

    fn cursor_up_by(&mut self, count: usize) -> &mut Self {
        self.position = self
            .position
            .try_up(count)
            .expect("Expected to be able to move the cursor up");
        self
    }

    fn try_cursor_left(&mut self) -> bool {
        if let Some(new_position) = self.position.try_left(1) {
            self.position = new_position;
            true
        } else {
            false
        }
    }

    fn try_cursor_left_by(&mut self, count: usize) -> bool {
        if let Some(new_position) = self.position.try_left(count) {
            self.position = new_position;
            true
        } else {
            false
        }
    }

    fn cursor_left(&mut self) -> &mut Self {
        self.position = self
            .position
            .try_left(1)
            .expect("Expected to be able to move the cursor left");
        self
    }

    fn cursor_left_by(&mut self, count: usize) -> &mut Self {
        self.position = self
            .position
            .try_left(count)
            .expect("Expected to be able to move the cursor left");
        self
    }

    fn cursor_right(&mut self) -> &mut Self {
        self.position = self.position.right(1);
        self
    }

    fn cursor_right_by(&mut self, count: usize) -> &mut Self {
        self.position = self.position.right(count);
        self
    }
}
