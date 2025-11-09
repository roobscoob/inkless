/// Determines where the ellipsis (`…`) should appear when text
/// is truncated due to overflow.
///
/// This allows different stylistic choices for indicating omitted content.
/// For example, truncating at the start (Left) emphasizes the most recent
/// portion of the text, while truncating at the end (Right) emphasizes the
/// beginning or prefix of the text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EllipsisPosition {
    /// Place the ellipsis at the start of the visible text (e.g. `…ending`).
    /// Useful when the rightmost content is more important, such as filenames.
    Left,

    /// Place the ellipsis in the center of the text (e.g. `mid…end`).
    /// Useful when both the beginning and end are relevant and a balance is desired.
    Center,

    /// Place the ellipsis at the end of the text (e.g. `beginning…`).
    /// This is the most common truncation style.
    #[default]
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Overflow {
    /// Truncate any text that would be rendered beyond the right edge.
    /// No indication is given that clipping occurred.
    Clip,

    /// Truncate text at the last available cell and render an ellipsis (…)
    /// to indicate that more content existed but was not shown.
    Ellipsis(EllipsisPosition),

    /// Wrap lines at grapheme cluster boundaries when reaching the right edge.
    /// No text is lost; overflow continues on the next line (if there is one).
    #[default]
    GraphemeWrap,

    /// Prefer wrapping at word boundaries when possible; if a single word
    /// exceeds the line width, fall back to grapheme-based wrapping.
    WordWrap,

    /// Treat any horizontal overflow as an error.
    /// Rendering will stop and return an error from the `TagSink`/writer.
    Error,
}
