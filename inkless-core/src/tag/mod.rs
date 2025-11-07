use crate::tag::colors::{Ansi8Color, Ansi16Color, Ansi256Color, TrueColor};

pub mod colors;

pub trait Tag {
    /// Should return `core::any::type_name<Self>()`
    fn get_name(&self) -> &'static str;

    fn get_ansi8_color(&self) -> Ansi8Color;
    fn get_ansi16_color(&self) -> Ansi16Color;
    fn get_ansi256_color(&self) -> Ansi256Color;
    fn get_true_color(&self) -> TrueColor;
}
