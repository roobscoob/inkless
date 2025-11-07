#[derive(Debug, Clone, Copy)]
pub struct RenderPosition {
    line: usize,
    column: usize,
}

impl RenderPosition {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn line_start(line: usize) -> Self {
        Self::new(line, 0)
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn try_left(&self, count: usize) -> Option<Self> {
        if self.column < count {
            None
        } else {
            Some(Self::new(self.line, self.column - count))
        }
    }

    pub fn right(&self, count: usize) -> Self {
        Self::new(self.line, self.column + count)
    }

    pub fn try_up(&self, count: usize) -> Option<Self> {
        if self.line < count {
            None
        } else {
            Some(Self::new(self.line - count, self.column))
        }
    }

    pub fn down(&self, count: usize) -> Self {
        Self::new(self.line + count, self.column)
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}
