use core::fmt::Debug;

use crate::{
    buffer::{RenderBuffer, RenderDispatcher},
    canvas::{AmbiguityPolicy, ext::RenderBufferCanvasExt},
    grapheme::{gph, r#static::StaticGrapheme},
    render_position::RenderPosition,
    renderable::Renderable,
    tag::{Tag, sink::TagSink},
};

#[derive(Debug)]
pub struct StaticRenderBuffer<T: Tag, const CELLS: usize, const GRAPHEME_WIDTH: usize = 7> {
    cells: [Option<(T, StaticGrapheme<GRAPHEME_WIDTH>)>; CELLS],
    width: usize,
    offset: usize,
    lowest_written_line: usize,
    ambiguity_policy: AmbiguityPolicy,
}

impl<T: Tag, const CELLS: usize, const GRAPHEME_WIDTH: usize>
    StaticRenderBuffer<T, CELLS, GRAPHEME_WIDTH>
{
    pub fn new(width: usize, offset: usize, ambiguity_policy: AmbiguityPolicy) -> Self {
        Self {
            cells: core::array::from_fn(|_| None),
            width,
            offset,
            lowest_written_line: core::usize::MIN,
            ambiguity_policy,
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
        position.column() + c.width(self.ambiguity_policy()) <= self.width
    }

    fn set_cell(&mut self, position: RenderPosition, c: &gph, tag: T) -> bool {
        if !self.can_set_cell(position, &c) {
            return false;
        }

        if position.line() > self.lowest_written_line {
            self.lowest_written_line = position.line();
        }

        if let Some(idx) = self.index_of(position) {
            self.cells[idx] = Some((tag, StaticGrapheme::from_single_grapheme(c)));
        }

        true
    }

    fn width(&self) -> Option<usize> {
        Some(self.width)
    }

    fn ambiguity_policy(&self) -> AmbiguityPolicy {
        self.ambiguity_policy
    }
}

impl<'a, T: Tag + 'a, const CELLS: usize, const GRAPHEME_WIDTH: usize, R: Renderable<T>>
    RenderDispatcher<T, R> for StaticRenderBuffer<T, CELLS, GRAPHEME_WIDTH>
{
    fn render<S: TagSink<T>>(
        sink: impl Into<S>,
        renderable: R,
        width: usize,
        ambiguity_policy: AmbiguityPolicy,
    ) -> S::Result {
        let mut offset = 0;
        let mut sink = sink.into();

        loop {
            let mut buffer = Self::new(width, offset, ambiguity_policy);
            let mut canvas = buffer.canvas_at(RenderPosition::zero());

            if let Err(e) = renderable.render_into(&mut canvas) {
                return S::Result::from(e);
            }

            if buffer.is_empty() {
                break;
            }

            let count = buffer.width * buffer.height();
            let width = buffer.width;
            let height = buffer.height();
            let mut skip_count = 0;

            for (index, value) in buffer.cells.into_iter().enumerate() {
                if skip_count > 0 {
                    skip_count -= 1;
                    continue;
                }

                if index >= count {
                    break;
                }

                let result = if let Some((tag, grapheme)) = value {
                    skip_count = grapheme.width(ambiguity_policy) - 1;

                    sink.append(&*grapheme, tag)
                } else {
                    sink.gap()
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
