use core::marker::PhantomData;

use crate::theme::Theme;
use crate::{canvas::Canvas, tag::Tag};

use crate::{
    canvas::summary::CanvasSummary,
    grapheme::gph,
    render_position::RenderPosition,
    renderable::{Renderable, RenderableError},
};

/// A canvas that takes "outer" tags `Ot` and maps them through a `Theme<Ot>`
/// to some "inner" tag type `Th::Result`, then forwards to an inner canvas.
pub struct ThemeCanvas<'a, T: Tag, Th> {
    pub(crate) inner: &'a mut dyn Canvas<T>,
    pub(crate) theme: PhantomData<Th>,
}

impl<'a, Ot, Th> Canvas<Ot> for ThemeCanvas<'a, Th::Result, Th>
where
    Ot: Tag,
    Th: Theme<Ot>,
{
    fn write<'b>(
        &'b mut self,
        renderable: &dyn Renderable<Ot>,
    ) -> Result<CanvasSummary, RenderableError> {
        let start = self.get_position();
        renderable.render_into(self)?;
        let end = self.get_position();

        Ok(CanvasSummary {
            start_position: start,
            end_position: end,
        })
    }

    fn set_char(&mut self, ch: char, tag: Ot) -> bool {
        let themed = Th::translate(tag);
        self.inner.set_char(ch, themed)
    }

    fn set_gph(&mut self, v: &gph, tag: Ot) -> bool {
        let themed = Th::translate(tag);
        self.inner.set_gph(v, themed)
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
