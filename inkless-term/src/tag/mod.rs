pub mod default;
pub mod indirection;

use inkless_core::tag::{Tag, untagged::Untagged};

use crate::styles::{
    Ansi8Color, Ansi16Color, Ansi256Color, BlinkSpeed, Intensity, TrueColor, UnderlineStyle,
};

pub trait AnsiTag: Tag {
    fn get_ansi8_foreground_color(&self) -> Option<Ansi8Color> {
        None
    }

    fn get_ansi16_foreground_color(&self) -> Option<Ansi16Color> {
        None
    }

    fn get_ansi256_foreground_color(&self) -> Option<Ansi256Color> {
        None
    }

    fn get_true_foreground_color(&self) -> Option<TrueColor> {
        None
    }

    fn get_ansi8_background_color(&self) -> Option<Ansi8Color> {
        None
    }

    fn get_ansi16_background_color(&self) -> Option<Ansi16Color> {
        None
    }

    fn get_ansi256_background_color(&self) -> Option<Ansi256Color> {
        None
    }

    fn get_true_background_color(&self) -> Option<TrueColor> {
        None
    }

    fn get_underline(&self) -> Option<UnderlineStyle> {
        None
    }

    fn get_ansi256_underline_color(&self) -> Option<Ansi256Color> {
        None
    }

    fn get_true_underline_color(&self) -> Option<TrueColor> {
        None
    }

    fn get_intensity(&self) -> Intensity {
        Intensity::Normal
    }

    fn get_blink_speed(&self) -> Option<BlinkSpeed> {
        None
    }

    fn is_italic(&self) -> bool {
        false
    }

    fn is_concealed(&self) -> bool {
        false
    }

    fn is_strikethrough(&self) -> bool {
        false
    }

    fn hyperlink_url<'t>(&'t self) -> Option<&'t str>
    where
        Self: 't,
    {
        None
    }

    fn hyperlink_id<'t>(&'t self) -> Option<&'t str>
    where
        Self: 't,
    {
        None
    }
}

impl AnsiTag for Untagged {}
