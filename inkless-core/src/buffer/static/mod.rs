use crate::{
    buffer::{RenderBuffer, RenderDispatcher},
    canvas::{NoDefaultTag, ext::RenderBufferCanvasExt},
    grapheme::{gph, r#static::StaticGrapheme},
    render_position::RenderPosition,
    tag::Tag,
};

pub struct StaticRenderBuffer<T: Tag, const CELLS: usize, const GRAPHEME_WIDTH: usize = 7> {
    cells: [(StaticGrapheme<GRAPHEME_WIDTH>, Option<T>); CELLS],
    width: usize,
    offset: usize,
    lowest_written_line: usize,
}

impl<T: Tag, const CELLS: usize, const GRAPHEME_WIDTH: usize>
    StaticRenderBuffer<T, CELLS, GRAPHEME_WIDTH>
{
    pub fn new(width: usize, offset: usize) -> Self {
        Self {
            cells: core::array::from_fn(|_| {
                (StaticGrapheme::from_single_grapheme_str(" ").unwrap(), None)
            }),
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

    pub fn is_empty(&self) -> bool {
        self.offset > self.lowest_written_line
    }
}

impl<T: Tag, const CELLS: usize, const GRAPHEME_WIDTH: usize> RenderBuffer<T>
    for StaticRenderBuffer<T, CELLS, GRAPHEME_WIDTH>
{
    fn can_set_cell(&self, position: RenderPosition, c: &gph) -> bool {
        position.column() + c.width() <= self.width
    }

    fn set_tagged_cell(&mut self, position: RenderPosition, c: &gph, tag: T) -> bool {
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

impl<T: Tag, const CELLS: usize, const GRAPHEME_WIDTH: usize> RenderDispatcher<T>
    for StaticRenderBuffer<T, CELLS, GRAPHEME_WIDTH>
{
    fn render<
        R: crate::renderable::Renderable<T, NoDefaultTag>,
        S: crate::tag::sink::TagSink<T>,
    >(
        sink: impl Into<S>,
        renderable: R,
        width: usize,
    ) -> S::Result {
        let mut offset = 0;
        let mut sink = sink.into();

        loop {
            let mut buffer = Self::new(width, offset);
            let mut canvas = buffer.canvas_at(RenderPosition::zero());

            renderable.render_into(&mut canvas).unwrap();

            if buffer.is_empty() {
                break;
            }

            let count = buffer.width * buffer.height();
            let width = buffer.width;
            let height = buffer.height();
            for (index, (grapheme, tag)) in buffer.cells.into_iter().enumerate() {
                if index >= count {
                    break;
                }

                let result = if let Some(tag) = tag {
                    sink.append_tagged(&*grapheme, tag)
                } else {
                    sink.append(&*grapheme)
                };

                match result {
                    core::ops::ControlFlow::Break(_) => break,
                    core::ops::ControlFlow::Continue(_) => {}
                }

                if index % width == width - 1 {
                    match sink.finalize_line() {
                        core::ops::ControlFlow::Break(_) => break,
                        core::ops::ControlFlow::Continue(_) => {}
                    }
                }
            }

            offset += height;
        }

        sink.finalize()
    }
}
