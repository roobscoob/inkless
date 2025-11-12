use crate::render_position::RenderPosition;

#[derive(Clone, Copy)]
pub struct CanvasSummary {
    pub start_position: RenderPosition,
    pub end_position: RenderPosition,
}

impl CanvasSummary {
    pub fn get_line_height(&self) -> usize {
        1 + (self.end_position.line() - self.start_position.line())
    }
}
