use inkless_core::writer::character::CharacterWriter;

use crate::styles::{Ansi8Color, Ansi16Color, Ansi256Color, TrueColor};

pub trait StrIgnoreCaseExt {
    /// Returns true if `self` ends with `suffix`, ignoring case.
    ///
    /// - `no_std`, `no_alloc`
    /// - Uses `char::to_lowercase` and compares the resulting
    ///   char sequences lazily.
    ///
    /// Note: This is *Unicode lowercase* based, not full Unicode
    /// "case folding". Edge cases like "ß" vs "SS" still won't
    /// be considered equal without a real case-folding table.
    fn ends_with_ignore_ascii_case(&self, suffix: &str) -> bool;
}

fn to_ascii_lower(b: u8) -> u8 {
    // ASCII 'A'..='Z' → 'a'..='z'
    if b'A' <= b && b <= b'Z' {
        b + (b'a' - b'A')
    } else {
        b
    }
}

impl StrIgnoreCaseExt for str {
    fn ends_with_ignore_ascii_case(&self, suffix: &str) -> bool {
        let h = self.as_bytes();
        let n = suffix.as_bytes();

        let h_len = h.len();
        let n_len = n.len();

        if n_len == 0 {
            return true;
        }
        if n_len > h_len {
            return false;
        }

        let start = h_len - n_len;

        for i in 0..n_len {
            if to_ascii_lower(h[start + i]) != to_ascii_lower(n[i]) {
                return false;
            }
        }

        true
    }
}

pub fn write_ansi8_fg<W: CharacterWriter>(
    writer: &mut W,
    color: Ansi8Color,
) -> Result<(), W::Error> {
    let seq = match color {
        Ansi8Color::Black => "\x1b[30m",
        Ansi8Color::Red => "\x1b[31m",
        Ansi8Color::Green => "\x1b[32m",
        Ansi8Color::Yellow => "\x1b[33m",
        Ansi8Color::Blue => "\x1b[34m",
        Ansi8Color::Magenta => "\x1b[35m",
        Ansi8Color::Cyan => "\x1b[36m",
        Ansi8Color::White => "\x1b[37m",
    };
    writer.write_str(seq)
}

pub fn write_ansi16_fg<W: CharacterWriter>(
    writer: &mut W,
    color: Ansi16Color,
) -> Result<(), W::Error> {
    let seq = match color {
        Ansi16Color::Normal(c) => match c {
            Ansi8Color::Black => "\x1b[30m",
            Ansi8Color::Red => "\x1b[31m",
            Ansi8Color::Green => "\x1b[32m",
            Ansi8Color::Yellow => "\x1b[33m",
            Ansi8Color::Blue => "\x1b[34m",
            Ansi8Color::Magenta => "\x1b[35m",
            Ansi8Color::Cyan => "\x1b[36m",
            Ansi8Color::White => "\x1b[37m",
        },
        Ansi16Color::Bright(c) => match c {
            Ansi8Color::Black => "\x1b[90m",
            Ansi8Color::Red => "\x1b[91m",
            Ansi8Color::Green => "\x1b[92m",
            Ansi8Color::Yellow => "\x1b[93m",
            Ansi8Color::Blue => "\x1b[94m",
            Ansi8Color::Magenta => "\x1b[95m",
            Ansi8Color::Cyan => "\x1b[96m",
            Ansi8Color::White => "\x1b[97m",
        },
    };
    writer.write_str(seq)
}

fn write_u8_dec<W: CharacterWriter>(writer: &mut W, mut n: u8) -> Result<(), W::Error> {
    let mut buf = [0u8; 3];
    let mut len = 0;

    if n == 0 {
        buf[0] = b'0';
        len = 1;
    } else {
        // First write digits in reverse into `tmp`
        let mut tmp = [0u8; 3];
        while n > 0 {
            tmp[len] = b'0' + (n % 10) as u8;
            len += 1;
            n /= 10;
        }

        // Now reverse them into `buf` so they're in the right order
        for i in 0..len {
            buf[i] = tmp[len - 1 - i];
        }
    }

    // &buf[..len] contains only '0'..='9'
    let s = str::from_utf8(&buf[..len]).unwrap();
    writer.write_str(s)
}

fn ansi8_index(color: Ansi8Color) -> u8 {
    match color {
        Ansi8Color::Black => 0,
        Ansi8Color::Red => 1,
        Ansi8Color::Green => 2,
        Ansi8Color::Yellow => 3,
        Ansi8Color::Blue => 4,
        Ansi8Color::Magenta => 5,
        Ansi8Color::Cyan => 6,
        Ansi8Color::White => 7,
    }
}

fn ansi256_index(color: Ansi256Color) -> u8 {
    let idx: u16 = match color {
        Ansi256Color::Normal(c) => ansi8_index(c) as u16,
        Ansi256Color::Bright(c) => 8 + ansi8_index(c) as u16,
        Ansi256Color::Rgb(r, g, b) => {
            // Assume r,g,b are 0..=5; if not, your code can clamp.
            16 + 36 * r as u16 + 6 * g as u16 + b as u16
        }
        Ansi256Color::Grayscale(v) => {
            // v is 0..=23
            232 + v as u16
        }
    };

    debug_assert!(idx <= 255);
    idx as u8
}

pub fn write_ansi256_fg<W: CharacterWriter>(
    writer: &mut W,
    color: Ansi256Color,
) -> Result<(), W::Error> {
    let idx = ansi256_index(color);

    writer.write_str("\x1b[38;5;")?;
    write_u8_dec(writer, idx)?;
    writer.write_str("m")
}

pub fn write_truecolor_fg<W: CharacterWriter>(
    writer: &mut W,
    TrueColor(r, g, b): TrueColor,
) -> Result<(), W::Error> {
    writer.write_str("\x1b[38;2;")?;
    write_u8_dec(writer, r)?;
    writer.write_str(";")?;
    write_u8_dec(writer, g)?;
    writer.write_str(";")?;
    write_u8_dec(writer, b)?;
    writer.write_str("m")
}

pub fn write_ansi8_bg<W: CharacterWriter>(
    writer: &mut W,
    color: Ansi8Color,
) -> Result<(), W::Error> {
    let seq = match color {
        Ansi8Color::Black => "\x1b[40m",
        Ansi8Color::Red => "\x1b[41m",
        Ansi8Color::Green => "\x1b[42m",
        Ansi8Color::Yellow => "\x1b[43m",
        Ansi8Color::Blue => "\x1b[44m",
        Ansi8Color::Magenta => "\x1b[45m",
        Ansi8Color::Cyan => "\x1b[46m",
        Ansi8Color::White => "\x1b[47m",
    };
    writer.write_str(seq)
}

pub fn write_ansi16_bg<W: CharacterWriter>(
    writer: &mut W,
    color: Ansi16Color,
) -> Result<(), W::Error> {
    let seq = match color {
        Ansi16Color::Normal(c) => match c {
            Ansi8Color::Black => "\x1b[40m",
            Ansi8Color::Red => "\x1b[41m",
            Ansi8Color::Green => "\x1b[42m",
            Ansi8Color::Yellow => "\x1b[43m",
            Ansi8Color::Blue => "\x1b[44m",
            Ansi8Color::Magenta => "\x1b[45m",
            Ansi8Color::Cyan => "\x1b[46m",
            Ansi8Color::White => "\x1b[47m",
        },
        Ansi16Color::Bright(c) => match c {
            Ansi8Color::Black => "\x1b[100m",
            Ansi8Color::Red => "\x1b[101m",
            Ansi8Color::Green => "\x1b[102m",
            Ansi8Color::Yellow => "\x1b[103m",
            Ansi8Color::Blue => "\x1b[104m",
            Ansi8Color::Magenta => "\x1b[105m",
            Ansi8Color::Cyan => "\x1b[106m",
            Ansi8Color::White => "\x1b[107m",
        },
    };
    writer.write_str(seq)
}

pub fn write_ansi256_bg<W: CharacterWriter>(
    writer: &mut W,
    color: Ansi256Color,
) -> Result<(), W::Error> {
    let idx = ansi256_index(color); // same logic as fg

    writer.write_str("\x1b[48;5;")?;
    write_u8_dec(writer, idx)?;
    writer.write_str("m")
}

pub fn write_truecolor_bg<W: CharacterWriter>(
    writer: &mut W,
    TrueColor(r, g, b): TrueColor,
) -> Result<(), W::Error> {
    writer.write_str("\x1b[48;2;")?;
    write_u8_dec(writer, r)?;
    writer.write_str(";")?;
    write_u8_dec(writer, g)?;
    writer.write_str(";")?;
    write_u8_dec(writer, b)?;
    writer.write_str("m")
}

pub fn write_ansi256_underline_color<W: CharacterWriter>(
    writer: &mut W,
    color: Ansi256Color,
) -> Result<(), W::Error> {
    let idx = ansi256_index(color); // same helper you use for fg/bg

    writer.write_str("\x1b[58;5;")?;
    write_u8_dec(writer, idx)?;
    writer.write_str("m")
}

pub fn write_truecolor_underline_color<W: CharacterWriter>(
    writer: &mut W,
    TrueColor(r, g, b): TrueColor,
) -> Result<(), W::Error> {
    writer.write_str("\x1b[58;2;")?;
    write_u8_dec(writer, r)?;
    writer.write_str(";")?;
    write_u8_dec(writer, g)?;
    writer.write_str(";")?;
    write_u8_dec(writer, b)?;
    writer.write_str("m")
}
