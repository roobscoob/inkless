use crate::{
    buffer::RenderBuffer,
    grapheme::{gph, r#static::StaticGrapheme},
    render_position::RenderPosition,
    tag::Tag,
};

pub struct StaticRenderBuffer<'tag, const CELLS: usize, const GRAPHEME_WIDTH: usize = 7> {
    cells: [(StaticGrapheme<GRAPHEME_WIDTH>, Option<&'tag dyn Tag>); CELLS],
    width: usize,
    offset: usize,
    lowest_written_line: usize,
}

impl<'tag, const CELLS: usize, const GRAPHEME_WIDTH: usize>
    StaticRenderBuffer<'tag, CELLS, GRAPHEME_WIDTH>
{
    pub fn new(width: usize, offset: usize) -> Self {
        Self {
            cells: [(StaticGrapheme::from_single_grapheme_str(" ").unwrap(), None); CELLS],
            width,
            offset,
            lowest_written_line: core::usize::MIN,
        }
    }

    fn height(&self) -> usize {
        CELLS / self.width
    }

    fn index_of(&self, pos: RenderPosition) -> Option<usize> {
        let line = pos.line() as isize - self.offset as isize;

        if line < 0 || line >= self.height() as isize || pos.column() >= self.width {
            None
        } else {
            Some(line as usize * self.width + pos.column())
        }
    }
}

impl<'tag, const CELLS: usize, const GRAPHEME_WIDTH: usize> RenderBuffer<'tag>
    for StaticRenderBuffer<'tag, CELLS, GRAPHEME_WIDTH>
{
    fn can_set_cell(&self, position: RenderPosition, c: &gph) -> bool {
        position.column() + c.width() <= self.width
    }

    fn set_tagged_cell(&mut self, position: RenderPosition, c: &gph, tag: &'tag dyn Tag) -> bool {
        if !self.can_set_cell(position, &c) {
            return false;
        }

        if position.line() > self.lowest_written_line {
            self.lowest_written_line = position.line();
        }

        if let Some(idx) = self.index_of(position) {
            self.cells[idx] = (StaticGrapheme::from_single_grapheme(c), Some(tag));
            true
        } else {
            false
        }
    }

    fn set_untagged_cell(&mut self, position: RenderPosition, c: &gph) -> bool {
        if !self.can_set_cell(position, &c) {
            return false;
        }

        if position.line() > self.lowest_written_line {
            self.lowest_written_line = position.line();
        }

        if let Some(idx) = self.index_of(position) {
            self.cells[idx] = (StaticGrapheme::from_single_grapheme(c), None);
            true
        } else {
            false
        }
    }

    fn width(&self) -> Option<usize> {
        Some(self.width)
    }
}
