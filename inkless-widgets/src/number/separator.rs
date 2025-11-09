use inkless_core::grapheme::gph;

/// Configuration for grouping separators, e.g. `10 92 81`.
///
/// Groups are counted from the *right* (least-significant digit).
/// For example, `every = 2` will format `109281` as `10 92 81`.
#[derive(Clone, Debug)]
pub struct NumberSeparator {
    /// How many digits per group, counting from the right. Must be > 0.
    pub every: usize,
    /// Grapheme used as the separator (e.g. `gph!(" ")` or `gph!(",")`).
    pub glyph: &'static gph,
}
