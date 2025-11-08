#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansi8Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansi16Color {
    Normal(Ansi8Color),
    Bright(Ansi8Color),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansi256Color {
    Normal(Ansi8Color),
    Bright(Ansi8Color),
    Rgb(u8, u8, u8),
    Grayscale(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrueColor(pub u8, pub u8, pub u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderlineStyle {
    Single,
    Double,
    Curly,
    Dotted,
    Dashed,
}

#[derive(Clone, Copy, Default)]
pub enum Intensity {
    Faint,

    #[default]
    Normal,

    Bold,
}

pub enum BlinkSpeed {
    Rapid,
    Slow,
}
