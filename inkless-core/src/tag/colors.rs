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

pub enum Ansi16Color {
    Normal(Ansi8Color),
    Bright(Ansi8Color),
}

pub enum Ansi256Color {
    Normal(Ansi8Color),
    Bright(Ansi8Color),
    Rgb(u8, u8, u8),
    Grayscale(u8),
}

pub struct TrueColor(u8, u8, u8);
