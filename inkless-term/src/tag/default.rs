use inkless_core::tag::Tag;

use crate::{
    styles::{
        Ansi8Color, Ansi16Color, Ansi256Color, BlinkSpeed, Intensity, TrueColor, UnderlineStyle,
    },
    tag::AnsiTag,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Ansi {
    // Foreground
    ansi8_fg: Option<Ansi8Color>,
    ansi16_fg: Option<Ansi16Color>,
    ansi256_fg: Option<Ansi256Color>,
    true_fg: Option<TrueColor>,

    // Background
    ansi8_bg: Option<Ansi8Color>,
    ansi16_bg: Option<Ansi16Color>,
    ansi256_bg: Option<Ansi256Color>,
    true_bg: Option<TrueColor>,

    // Underline
    underline: Option<UnderlineStyle>,
    ansi256_underline: Option<Ansi256Color>,
    true_underline: Option<TrueColor>,

    // Other attributes
    intensity: Intensity,
    blink: Option<BlinkSpeed>,
    italic: bool,
    concealed: bool,
    strikethrough: bool,
}

impl Ansi {
    pub const fn new() -> Self {
        Self {
            ansi8_fg: None,
            ansi16_fg: None,
            ansi256_fg: None,
            true_fg: None,

            ansi8_bg: None,
            ansi16_bg: None,
            ansi256_bg: None,
            true_bg: None,

            underline: None,
            ansi256_underline: None,
            true_underline: None,

            intensity: Intensity::Normal,
            blink: None,
            italic: false,
            concealed: false,
            strikethrough: false,
        }
    }

    /// A completely "unstyled" tag.
    pub const fn plain() -> Self {
        Self::new()
    }

    // --- Convenience constructors -----------------------------------------

    pub const fn black(self) -> Self {
        self.fg8(Ansi8Color::Black)
            .fg16(Ansi16Color::Normal(Ansi8Color::Black))
    }

    pub const fn red(self) -> Self {
        self.fg8(Ansi8Color::Red)
            .fg16(Ansi16Color::Normal(Ansi8Color::Red))
    }

    pub const fn green(self) -> Self {
        self.fg8(Ansi8Color::Green)
            .fg16(Ansi16Color::Normal(Ansi8Color::Green))
    }

    pub const fn yellow(self) -> Self {
        self.fg8(Ansi8Color::Yellow)
            .fg16(Ansi16Color::Normal(Ansi8Color::Yellow))
    }

    pub const fn blue(self) -> Self {
        self.fg8(Ansi8Color::Blue)
            .fg16(Ansi16Color::Normal(Ansi8Color::Blue))
    }

    pub const fn magenta(self) -> Self {
        self.fg8(Ansi8Color::Magenta)
            .fg16(Ansi16Color::Normal(Ansi8Color::Magenta))
    }

    pub const fn cyan(self) -> Self {
        self.fg8(Ansi8Color::Cyan)
            .fg16(Ansi16Color::Normal(Ansi8Color::Cyan))
    }

    pub const fn white(self) -> Self {
        self.fg8(Ansi8Color::White)
            .fg16(Ansi16Color::Normal(Ansi8Color::White))
    }

    pub const fn bright_black(self) -> Self {
        self.fg16(Ansi16Color::Bright(Ansi8Color::Black))
    }

    pub const fn bright_red(self) -> Self {
        self.fg16(Ansi16Color::Bright(Ansi8Color::Red))
    }

    pub const fn bright_green(self) -> Self {
        self.fg16(Ansi16Color::Bright(Ansi8Color::Green))
    }

    pub const fn bright_yellow(self) -> Self {
        self.fg16(Ansi16Color::Bright(Ansi8Color::Yellow))
    }

    pub const fn bright_blue(self) -> Self {
        self.fg16(Ansi16Color::Bright(Ansi8Color::Blue))
    }

    pub const fn bright_magenta(self) -> Self {
        self.fg16(Ansi16Color::Bright(Ansi8Color::Magenta))
    }

    pub const fn bright_cyan(self) -> Self {
        self.fg16(Ansi16Color::Bright(Ansi8Color::Cyan))
    }

    pub const fn bright_white(self) -> Self {
        self.fg16(Ansi16Color::Bright(Ansi8Color::White))
    }

    pub const fn bg_black(self) -> Self {
        self.bg8(Ansi8Color::Black)
            .bg16(Ansi16Color::Normal(Ansi8Color::Black))
    }

    pub const fn bg_red(self) -> Self {
        self.bg8(Ansi8Color::Red)
            .bg16(Ansi16Color::Normal(Ansi8Color::Red))
    }

    pub const fn bg_green(self) -> Self {
        self.bg8(Ansi8Color::Green)
            .bg16(Ansi16Color::Normal(Ansi8Color::Green))
    }

    pub const fn bg_yellow(self) -> Self {
        self.bg8(Ansi8Color::Yellow)
            .bg16(Ansi16Color::Normal(Ansi8Color::Yellow))
    }

    pub const fn bg_blue(self) -> Self {
        self.bg8(Ansi8Color::Blue)
            .bg16(Ansi16Color::Normal(Ansi8Color::Blue))
    }

    pub const fn bg_magenta(self) -> Self {
        self.bg8(Ansi8Color::Magenta)
            .bg16(Ansi16Color::Normal(Ansi8Color::Magenta))
    }

    pub const fn bg_cyan(self) -> Self {
        self.bg8(Ansi8Color::Cyan)
            .bg16(Ansi16Color::Normal(Ansi8Color::Cyan))
    }

    pub const fn bg_white(self) -> Self {
        self.bg8(Ansi8Color::White)
            .bg16(Ansi16Color::Normal(Ansi8Color::White))
    }

    pub const fn bg_bright_black(self) -> Self {
        self.bg16(Ansi16Color::Bright(Ansi8Color::Black))
    }

    pub const fn bg_bright_red(self) -> Self {
        self.bg16(Ansi16Color::Bright(Ansi8Color::Red))
    }

    pub const fn bg_bright_green(self) -> Self {
        self.bg16(Ansi16Color::Bright(Ansi8Color::Green))
    }

    pub const fn bg_bright_yellow(self) -> Self {
        self.bg16(Ansi16Color::Bright(Ansi8Color::Yellow))
    }

    pub const fn bg_bright_blue(self) -> Self {
        self.bg16(Ansi16Color::Bright(Ansi8Color::Blue))
    }

    pub const fn bg_bright_magenta(self) -> Self {
        self.bg16(Ansi16Color::Bright(Ansi8Color::Magenta))
    }

    pub const fn bg_bright_cyan(self) -> Self {
        self.bg16(Ansi16Color::Bright(Ansi8Color::Cyan))
    }

    pub const fn bg_bright_white(self) -> Self {
        self.bg16(Ansi16Color::Bright(Ansi8Color::White))
    }

    // --- Builder-style setters for all the knobs -------------------------

    pub const fn fg8(mut self, color: Ansi8Color) -> Self {
        self.ansi8_fg = Some(color);
        self
    }

    pub const fn fg16(mut self, color: Ansi16Color) -> Self {
        self.ansi16_fg = Some(color);
        self
    }

    pub const fn fg256(mut self, color: Ansi256Color) -> Self {
        self.ansi256_fg = Some(color);
        self
    }

    pub const fn fg_true(mut self, color: TrueColor) -> Self {
        self.true_fg = Some(color);
        self
    }

    pub const fn bg8(mut self, color: Ansi8Color) -> Self {
        self.ansi8_bg = Some(color);
        self
    }

    pub const fn bg16(mut self, color: Ansi16Color) -> Self {
        self.ansi16_bg = Some(color);
        self
    }

    pub const fn bg256(mut self, color: Ansi256Color) -> Self {
        self.ansi256_bg = Some(color);
        self
    }

    pub const fn bg_true(mut self, color: TrueColor) -> Self {
        self.true_bg = Some(color);
        self
    }

    pub const fn underline(mut self, style: UnderlineStyle) -> Self {
        self.underline = Some(style);
        self
    }

    pub const fn underline_256(mut self, color: Ansi256Color) -> Self {
        self.ansi256_underline = Some(color);
        self
    }

    pub const fn underline_true(mut self, color: TrueColor) -> Self {
        self.true_underline = Some(color);
        self
    }

    pub const fn intensity(mut self, intensity: Intensity) -> Self {
        self.intensity = intensity;
        self
    }

    pub const fn blink(mut self, speed: BlinkSpeed) -> Self {
        self.blink = Some(speed);
        self
    }

    pub const fn italic(mut self, value: bool) -> Self {
        self.italic = value;
        self
    }

    pub const fn concealed(mut self, value: bool) -> Self {
        self.concealed = value;
        self
    }

    pub const fn strikethrough(mut self, value: bool) -> Self {
        self.strikethrough = value;
        self
    }

    // Nice sugar:
    pub const fn bold(self) -> Self {
        self.intensity(Intensity::Bold)
    }

    pub const fn faint(self) -> Self {
        self.intensity(Intensity::Faint)
    }

    pub const fn no_blink(mut self) -> Self {
        self.blink = None;
        self
    }
}

// This makes `Ansi` usable as a tag in your render system.
impl Tag for Ansi {}

impl AnsiTag for Ansi {
    fn get_ansi8_foreground_color(&self) -> Option<Ansi8Color> {
        self.ansi8_fg
    }

    fn get_ansi16_foreground_color(&self) -> Option<Ansi16Color> {
        self.ansi16_fg
    }

    fn get_ansi256_foreground_color(&self) -> Option<Ansi256Color> {
        self.ansi256_fg
    }

    fn get_true_foreground_color(&self) -> Option<TrueColor> {
        self.true_fg
    }

    fn get_ansi8_background_color(&self) -> Option<Ansi8Color> {
        self.ansi8_bg
    }

    fn get_ansi16_background_color(&self) -> Option<Ansi16Color> {
        self.ansi16_bg
    }

    fn get_ansi256_background_color(&self) -> Option<Ansi256Color> {
        self.ansi256_bg
    }

    fn get_true_background_color(&self) -> Option<TrueColor> {
        self.true_bg
    }

    fn get_underline(&self) -> Option<UnderlineStyle> {
        self.underline
    }

    fn get_ansi256_underline_color(&self) -> Option<Ansi256Color> {
        self.ansi256_underline
    }

    fn get_true_underline_color(&self) -> Option<TrueColor> {
        self.true_underline
    }

    fn get_intensity(&self) -> Intensity {
        self.intensity
    }

    fn get_blink_speed(&self) -> Option<BlinkSpeed> {
        self.blink
    }

    fn is_italic(&self) -> bool {
        self.italic
    }

    fn is_concealed(&self) -> bool {
        self.concealed
    }

    fn is_strikethrough(&self) -> bool {
        self.strikethrough
    }
}
