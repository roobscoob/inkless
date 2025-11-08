use inkless_core::writer::character::CharacterWriter;

use crate::{
    styles::{
        Ansi8Color, Ansi16Color, Ansi256Color, BlinkSpeed, Intensity, TrueColor, UnderlineStyle,
    },
    support::{AnsiColorSupport, AnsiSupport, AnsiUnderlineColorSupport},
    tag::AnsiTag,
    utils::{
        write_ansi8_bg, write_ansi8_fg, write_ansi16_bg, write_ansi16_fg, write_ansi256_bg,
        write_ansi256_fg, write_ansi256_underline_color, write_truecolor_bg, write_truecolor_fg,
        write_truecolor_underline_color,
    },
};

pub fn write_intensity_delta<W: CharacterWriter, T: AnsiTag>(
    writer: &mut W,
    old: Option<&T>,
    new: Option<&T>,
) -> Result<(), W::Error> {
    let old = old.map(AnsiTag::get_intensity).unwrap_or_default();
    let new = new.map(AnsiTag::get_intensity).unwrap_or_default();

    Ok(match (old, new) {
        (Intensity::Bold, Intensity::Bold)
        | (Intensity::Normal, Intensity::Normal)
        | (Intensity::Faint, Intensity::Faint) => {}

        (_, Intensity::Bold) => writer.write_str("\x1b[1m")?,
        (_, Intensity::Faint) => writer.write_str("\x1b[2m")?,
        (_, Intensity::Normal) => writer.write_str("\x1b[22m")?,
    })
}

pub fn write_blink_delta<W: CharacterWriter, T: AnsiTag>(
    writer: &mut W,
    old: Option<&T>,
    new: Option<&T>,
) -> Result<(), W::Error> {
    let old = old.and_then(AnsiTag::get_blink_speed);
    let new = new.and_then(AnsiTag::get_blink_speed);

    Ok(match (old, new) {
        (Some(BlinkSpeed::Slow), Some(BlinkSpeed::Slow))
        | (Some(BlinkSpeed::Rapid), Some(BlinkSpeed::Rapid))
        | (None, None) => {}

        (_, Some(BlinkSpeed::Slow)) => writer.write_str("\x1b[5m")?,
        (_, Some(BlinkSpeed::Rapid)) => writer.write_str("\x1b[6m")?,
        (_, None) => writer.write_str("\x1b[25m")?,
    })
}

pub fn write_italic_delta<W: CharacterWriter, T: AnsiTag>(
    writer: &mut W,
    old: Option<&T>,
    new: Option<&T>,
) -> Result<(), W::Error> {
    let old = old.map(AnsiTag::is_italic).unwrap_or(false);
    let new = new.map(AnsiTag::is_italic).unwrap_or(false);

    Ok(match (old, new) {
        (true, true) | (false, false) => {}
        (_, true) => writer.write_str("\x1b[3m")?,
        (_, false) => writer.write_str("\x1b[23m")?,
    })
}

pub fn write_concealed_delta<W: CharacterWriter, T: AnsiTag>(
    writer: &mut W,
    old: Option<&T>,
    new: Option<&T>,
) -> Result<(), W::Error> {
    let old = old.map(AnsiTag::is_concealed).unwrap_or(false);
    let new = new.map(AnsiTag::is_concealed).unwrap_or(false);

    Ok(match (old, new) {
        (true, true) | (false, false) => {}
        (_, true) => writer.write_str("\x1b[8m")?,
        (_, false) => writer.write_str("\x1b[28m")?,
    })
}

pub fn write_strikethrough_delta<W: CharacterWriter, T: AnsiTag>(
    writer: &mut W,
    old: Option<&T>,
    new: Option<&T>,
) -> Result<(), W::Error> {
    let old = old.map(AnsiTag::is_strikethrough).unwrap_or(false);
    let new = new.map(AnsiTag::is_strikethrough).unwrap_or(false);

    Ok(match (old, new) {
        (true, true) | (false, false) => {}
        (_, true) => writer.write_str("\x1b[9m")?,
        (_, false) => writer.write_str("\x1b[29m")?,
    })
}

pub fn write_foreground_color_delta<W: CharacterWriter, T: AnsiTag>(
    writer: &mut W,
    support: AnsiSupport,
    old: Option<&T>,
    new: Option<&T>,
) -> Result<(), W::Error> {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum EffectiveFgColor {
        None,
        Ansi8(Ansi8Color),
        Ansi16(Ansi16Color),
        Ansi256(Ansi256Color),
        TrueColor(TrueColor),
    }

    fn resolve_effective<T: AnsiTag>(tag: &T, support: AnsiSupport) -> EffectiveFgColor {
        match support.color {
            AnsiColorSupport::TrueColor => {
                // Try most specific → least specific
                if let Some(c) = tag.get_true_foreground_color() {
                    EffectiveFgColor::TrueColor(c)
                } else if let Some(c) = tag.get_ansi256_foreground_color() {
                    EffectiveFgColor::Ansi256(c)
                } else if let Some(c) = tag.get_ansi16_foreground_color() {
                    EffectiveFgColor::Ansi16(c)
                } else if let Some(c) = tag.get_ansi8_foreground_color() {
                    EffectiveFgColor::Ansi8(c)
                } else {
                    EffectiveFgColor::None
                }
            }
            AnsiColorSupport::Ansi256 => {
                if let Some(c) = tag.get_ansi256_foreground_color() {
                    EffectiveFgColor::Ansi256(c)
                } else if let Some(c) = tag.get_ansi16_foreground_color() {
                    EffectiveFgColor::Ansi16(c)
                } else if let Some(c) = tag.get_ansi8_foreground_color() {
                    EffectiveFgColor::Ansi8(c)
                } else {
                    EffectiveFgColor::None
                }
            }
            AnsiColorSupport::Ansi16 => {
                if let Some(c) = tag.get_ansi16_foreground_color() {
                    EffectiveFgColor::Ansi16(c)
                } else if let Some(c) = tag.get_ansi8_foreground_color() {
                    EffectiveFgColor::Ansi8(c)
                } else {
                    EffectiveFgColor::None
                }
            }
            AnsiColorSupport::Ansi8 => {
                if let Some(c) = tag.get_ansi8_foreground_color() {
                    EffectiveFgColor::Ansi8(c)
                } else {
                    EffectiveFgColor::None
                }
            }
            AnsiColorSupport::None => EffectiveFgColor::None,
        }
    }

    // Resolve old/new into “what will actually be used”
    let old_eff = old
        .map(|t| resolve_effective(t, support))
        .unwrap_or(EffectiveFgColor::None);
    let new_eff = new
        .map(|t| resolve_effective(t, support))
        .unwrap_or(EffectiveFgColor::None);

    // If nothing changed, don’t emit anything.
    if old_eff == new_eff {
        return Ok(());
    }

    // Otherwise, emit the new state.
    Ok(match new_eff {
        EffectiveFgColor::None => writer.write_str("\x1b[39m")?,
        EffectiveFgColor::Ansi8(c) => write_ansi8_fg(writer, c)?,
        EffectiveFgColor::Ansi16(c) => write_ansi16_fg(writer, c)?,
        EffectiveFgColor::Ansi256(c) => write_ansi256_fg(writer, c)?,
        EffectiveFgColor::TrueColor(c) => write_truecolor_fg(writer, c)?,
    })
}

pub fn write_background_color_delta<W: CharacterWriter, T: AnsiTag>(
    writer: &mut W,
    support: AnsiSupport,
    old: Option<&T>,
    new: Option<&T>,
) -> Result<(), W::Error> {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum EffectiveBgColor {
        None,
        Ansi8(Ansi8Color),
        Ansi16(Ansi16Color),
        Ansi256(Ansi256Color),
        TrueColor(TrueColor),
    }

    fn resolve_effective<T: AnsiTag>(tag: &T, support: AnsiSupport) -> EffectiveBgColor {
        match support.color {
            AnsiColorSupport::TrueColor => {
                // Try most specific → least specific
                if let Some(c) = tag.get_true_background_color() {
                    EffectiveBgColor::TrueColor(c)
                } else if let Some(c) = tag.get_ansi256_background_color() {
                    EffectiveBgColor::Ansi256(c)
                } else if let Some(c) = tag.get_ansi16_background_color() {
                    EffectiveBgColor::Ansi16(c)
                } else if let Some(c) = tag.get_ansi8_background_color() {
                    EffectiveBgColor::Ansi8(c)
                } else {
                    EffectiveBgColor::None
                }
            }
            AnsiColorSupport::Ansi256 => {
                if let Some(c) = tag.get_ansi256_background_color() {
                    EffectiveBgColor::Ansi256(c)
                } else if let Some(c) = tag.get_ansi16_background_color() {
                    EffectiveBgColor::Ansi16(c)
                } else if let Some(c) = tag.get_ansi8_background_color() {
                    EffectiveBgColor::Ansi8(c)
                } else {
                    EffectiveBgColor::None
                }
            }
            AnsiColorSupport::Ansi16 => {
                if let Some(c) = tag.get_ansi16_background_color() {
                    EffectiveBgColor::Ansi16(c)
                } else if let Some(c) = tag.get_ansi8_background_color() {
                    EffectiveBgColor::Ansi8(c)
                } else {
                    EffectiveBgColor::None
                }
            }
            AnsiColorSupport::Ansi8 => {
                if let Some(c) = tag.get_ansi8_background_color() {
                    EffectiveBgColor::Ansi8(c)
                } else {
                    EffectiveBgColor::None
                }
            }
            AnsiColorSupport::None => EffectiveBgColor::None,
        }
    }

    // Resolve old/new into “what will actually be used”
    let old_eff = old
        .map(|t| resolve_effective(t, support))
        .unwrap_or(EffectiveBgColor::None);
    let new_eff = new
        .map(|t| resolve_effective(t, support))
        .unwrap_or(EffectiveBgColor::None);

    // If nothing changed, don’t emit anything.
    if old_eff == new_eff {
        return Ok(());
    }

    // Otherwise, emit the new state.
    Ok(match new_eff {
        // reset background to default
        EffectiveBgColor::None => writer.write_str("\x1b[49m")?,
        EffectiveBgColor::Ansi8(c) => write_ansi8_bg(writer, c)?,
        EffectiveBgColor::Ansi16(c) => write_ansi16_bg(writer, c)?,
        EffectiveBgColor::Ansi256(c) => write_ansi256_bg(writer, c)?,
        EffectiveBgColor::TrueColor(c) => write_truecolor_bg(writer, c)?,
    })
}

pub fn write_underline_style_delta<W: CharacterWriter, T: AnsiTag>(
    writer: &mut W,
    old: Option<&T>,
    new: Option<&T>,
) -> Result<(), W::Error> {
    let old_style = old.and_then(AnsiTag::get_underline);
    let new_style = new.and_then(AnsiTag::get_underline);

    // No change → no output
    if old_style == new_style {
        return Ok(());
    }

    Ok(match new_style {
        None => writer.write_str("\x1b[24m")?,
        Some(UnderlineStyle::Single) => writer.write_str("\x1b[4m")?,
        Some(UnderlineStyle::Double) => writer.write_str("\x1b[4:2m")?,
        Some(UnderlineStyle::Curly) => writer.write_str("\x1b[4:3m")?,
        Some(UnderlineStyle::Dotted) => writer.write_str("\x1b[4:4m")?,
        Some(UnderlineStyle::Dashed) => writer.write_str("\x1b[4:5m")?,
    })
}

pub fn write_underline_color_delta<W: CharacterWriter, T: AnsiTag>(
    writer: &mut W,
    support: AnsiSupport,
    old: Option<&T>,
    new: Option<&T>,
) -> Result<(), W::Error> {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum EffectiveUnderlineColor {
        None,
        Ansi256(Ansi256Color),
        TrueColor(TrueColor),
    }

    fn resolve_effective<T: AnsiTag>(tag: &T, support: AnsiSupport) -> EffectiveUnderlineColor {
        match support.underline {
            AnsiUnderlineColorSupport::TrueColor => {
                if let Some(c) = tag.get_true_underline_color() {
                    EffectiveUnderlineColor::TrueColor(c)
                } else if let Some(c) = tag.get_ansi256_underline_color() {
                    EffectiveUnderlineColor::Ansi256(c)
                } else {
                    EffectiveUnderlineColor::None
                }
            }
            AnsiUnderlineColorSupport::Ansi256 => {
                if let Some(c) = tag.get_ansi256_underline_color() {
                    EffectiveUnderlineColor::Ansi256(c)
                } else {
                    EffectiveUnderlineColor::None
                }
            }
            AnsiUnderlineColorSupport::None => EffectiveUnderlineColor::None,
        }
    }

    let old_eff = old
        .map(|t| resolve_effective(t, support))
        .unwrap_or(EffectiveUnderlineColor::None);
    let new_eff = new
        .map(|t| resolve_effective(t, support))
        .unwrap_or(EffectiveUnderlineColor::None);

    // No change → no escape codes
    if old_eff == new_eff {
        return Ok(());
    }

    Ok(match new_eff {
        EffectiveUnderlineColor::None => {
            // reset underline color to default
            writer.write_str("\x1b[59m")?
        }
        EffectiveUnderlineColor::Ansi256(c) => write_ansi256_underline_color(writer, c)?,
        EffectiveUnderlineColor::TrueColor(c) => write_truecolor_underline_color(writer, c)?,
    })
}

pub fn write_hyperlink_delta<W: CharacterWriter, T: AnsiTag>(
    writer: &mut W,
    support: AnsiSupport,
    old: Option<&T>,
    new: Option<&T>,
) -> Result<(), W::Error> {
    if !support.hyperlinks {
        return Ok(());
    }

    let old_url = old.and_then(AnsiTag::hyperlink_url);
    let old_id = old.and_then(AnsiTag::hyperlink_id);
    let new_url = new.and_then(AnsiTag::hyperlink_url);
    let new_id = new.and_then(AnsiTag::hyperlink_id);

    // If URL and ID are both unchanged, no need to emit anything.
    if old_url == new_url && old_id == new_id {
        return Ok(());
    }

    // If there's no new URL, we close any existing hyperlink.
    if new_url.is_none() {
        // OSC 8 ; ; ST   → end hyperlink
        return writer.write_str("\x1b]8;;\x1b\\");
    }

    // Otherwise, we (re)open or change the hyperlink.
    let url = new_url.unwrap();

    // OSC 8 ; params ; URI ST
    writer.write_str("\x1b]8;")?;

    if let Some(id) = new_id {
        writer.write_str("id=")?;
        writer.write_str(id)?;
    }

    writer.write_str(";")?;
    writer.write_str(url)?;
    writer.write_str("\x1b\\")?;

    Ok(())
}
