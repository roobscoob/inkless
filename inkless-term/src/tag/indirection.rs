use inkless_core::tag::Tag;

use crate::{
    styles::{
        Ansi8Color, Ansi16Color, Ansi256Color, BlinkSpeed, Intensity, TrueColor, UnderlineStyle,
    },
    tag::AnsiTag,
};

pub trait AnsiReference: core::ops::Deref {}

impl<T: ?Sized> AnsiReference for &T {}
impl<T: ?Sized> AnsiReference for &mut T {}

#[cfg(feature = "alloc")]
impl<T: ?Sized> AnsiReference for alloc::boxed::Box<T> {}

#[cfg(feature = "std")]
impl<T: ?Sized> AnsiReference for std::rc::Rc<T> {}

#[cfg(feature = "std")]
impl<T: ?Sized> AnsiReference for std::sync::Arc<T> {}

impl<'a, T: AnsiTag + ?Sized, R: AnsiReference<Target = T> + Tag> AnsiTag for R {
    fn get_ansi8_foreground_color(&self) -> Option<Ansi8Color> {
        (**self).get_ansi8_foreground_color()
    }

    fn get_ansi16_foreground_color(&self) -> Option<Ansi16Color> {
        (**self).get_ansi16_foreground_color()
    }

    fn get_ansi256_foreground_color(&self) -> Option<Ansi256Color> {
        (**self).get_ansi256_foreground_color()
    }

    fn get_true_foreground_color(&self) -> Option<TrueColor> {
        (**self).get_true_foreground_color()
    }

    fn get_ansi8_background_color(&self) -> Option<Ansi8Color> {
        (**self).get_ansi8_background_color()
    }

    fn get_ansi16_background_color(&self) -> Option<Ansi16Color> {
        (**self).get_ansi16_background_color()
    }

    fn get_ansi256_background_color(&self) -> Option<Ansi256Color> {
        (**self).get_ansi256_background_color()
    }

    fn get_true_background_color(&self) -> Option<TrueColor> {
        (**self).get_true_background_color()
    }

    fn get_underline(&self) -> Option<UnderlineStyle> {
        (**self).get_underline()
    }

    fn get_ansi256_underline_color(&self) -> Option<Ansi256Color> {
        (**self).get_ansi256_underline_color()
    }

    fn get_true_underline_color(&self) -> Option<TrueColor> {
        (**self).get_true_underline_color()
    }

    fn get_intensity(&self) -> Intensity {
        (**self).get_intensity()
    }

    fn get_blink_speed(&self) -> Option<BlinkSpeed> {
        (**self).get_blink_speed()
    }

    fn is_italic(&self) -> bool {
        (**self).is_italic()
    }

    fn is_concealed(&self) -> bool {
        (**self).is_concealed()
    }

    fn is_strikethrough(&self) -> bool {
        (**self).is_strikethrough()
    }

    //TODO: Solve
    fn hyperlink_url<'b>(&'b self) -> Option<&'b str>
    where
        Self: 'b,
    {
        // (**self).hyperlink_url()
        None
    }

    //TODO: Solve
    fn hyperlink_id<'b>(&'b self) -> Option<&'b str>
    where
        Self: 'b,
    {
        // (**self).hyperlink_id()
        None
    }
}
