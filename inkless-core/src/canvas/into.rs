use crate::{
    canvas::{Canvas, summary::CanvasSummary},
    grapheme::gph,
    render_position::RenderPosition,
    renderable::{Renderable, RenderableError},
    tag::Tag,
};

pub struct IntoCanvas<'a, Tt: Tag> {
    pub(crate) inner: &'a mut dyn Canvas<Tt>,
}

impl<'a, Tt: Tag> IntoCanvas<'a, Tt> {
    pub fn new(inner: &'a mut dyn Canvas<Tt>) -> Self {
        Self { inner }
    }
}

impl<'a, Tt: Tag, Ot: Tag + Into<Tt>> Canvas<Ot> for IntoCanvas<'a, Tt> {
    fn write<'b>(
        &'b mut self,
        renderable: &dyn Renderable<Ot>,
    ) -> Result<CanvasSummary, RenderableError> {
        let start = self.inner.get_position();
        renderable.render_into(self)?;
        let end = self.inner.get_position();

        Ok(CanvasSummary {
            start_position: start,
            end_position: end,
        })
    }

    fn set_char(&mut self, ch: char, tag: Ot) -> bool {
        self.inner.set_char(ch, tag.into())
    }

    fn set_gph(&mut self, v: &gph, tag: Ot) -> bool {
        self.inner.set_gph(v, tag.into())
    }

    fn can_set_gph(&mut self, v: &gph) -> bool {
        // Width is independent of tag, so just forward.
        self.inner.can_set_gph(v)
    }

    fn get_start_position(&self) -> RenderPosition {
        self.inner.get_start_position()
    }

    fn get_position(&self) -> RenderPosition {
        self.inner.get_position()
    }

    fn set_position(&mut self, position: RenderPosition) -> &mut dyn Canvas<Ot> {
        self.inner.set_position(position);
        self
    }

    fn set_column(&mut self, column: usize) -> &mut dyn Canvas<Ot> {
        self.inner.set_column(column);
        self
    }

    fn set_line(&mut self, line: usize) -> &mut dyn Canvas<Ot> {
        self.inner.set_line(line);
        self
    }

    fn cursor_down(&mut self) -> &mut dyn Canvas<Ot> {
        self.inner.cursor_down();
        self
    }

    fn cursor_down_by(&mut self, count: usize) -> &mut dyn Canvas<Ot> {
        self.inner.cursor_down_by(count);
        self
    }

    fn try_cursor_up(&mut self) -> bool {
        self.inner.try_cursor_up()
    }

    fn try_cursor_up_by(&mut self, count: usize) -> bool {
        self.inner.try_cursor_up_by(count)
    }

    fn cursor_up(&mut self) -> &mut dyn Canvas<Ot> {
        self.inner.cursor_up();
        self
    }

    fn cursor_up_by(&mut self, count: usize) -> &mut dyn Canvas<Ot> {
        self.inner.cursor_up_by(count);
        self
    }

    fn try_cursor_left(&mut self) -> bool {
        self.inner.try_cursor_left()
    }

    fn try_cursor_left_by(&mut self, count: usize) -> bool {
        self.inner.try_cursor_left_by(count)
    }

    fn cursor_left(&mut self) -> &mut dyn Canvas<Ot> {
        self.inner.cursor_left();
        self
    }

    fn cursor_left_by(&mut self, count: usize) -> &mut dyn Canvas<Ot> {
        self.inner.cursor_left_by(count);
        self
    }

    fn cursor_right(&mut self) -> &mut dyn Canvas<Ot> {
        self.inner.cursor_right();
        self
    }

    fn cursor_right_by(&mut self, count: usize) -> &mut dyn Canvas<Ot> {
        self.inner.cursor_right_by(count);
        self
    }
}
