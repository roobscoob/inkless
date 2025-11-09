use inkless::{
    buffer::{RenderDispatcher, r#static::StaticRenderBuffer},
    canvas::AmbiguityPolicy,
    gph,
    number::{Number, tag::NumberTag},
    tag::untagged::Untagged,
    text::{Text, TextWithRenderable},
    theme::{Theme, ext::RenderableThemeExt},
};
use inkless_term::{sink::AnsiSink, styles::TrueColor, tag::default::Ansi};

const W: usize = 45;
type B<T> = StaticRenderBuffer<T, W>;

struct DemoTheme;

struct DemoThemeNumbers;

impl Theme<NumberTag> for DemoThemeNumbers {
    type Result = Ansi;

    fn translate(from: NumberTag) -> Self::Result {
        Ansi::plain().bg_red()
    }
}

impl Into<DemoThemeNumbers> for DemoTheme {
    fn into(self) -> DemoThemeNumbers {
        DemoThemeNumbers
    }
}

pub fn main() {
    let number = Number::new(1024u32);

    let sep = Number::new(1024u32).with_separator(3, gph!(","));

    let b16 = Number::new(1024u32).with_base(16).with_prefix("0x");

    let b16_sep = Number::new(1024u32)
        .with_base(16)
        .with_separator(2, gph!("_"))
        .with_prefix("0x");

    let n_b16_sep = Number::new(-1024i32)
        .with_base(16)
        .with_separator(2, gph!("_"))
        .with_prefix("0x")
        .theme::<DemoThemeNumbers>();

    let debug = Text::of::<Untagged>("Normal number:                      ")
        .with_component::<Untagged>(number);

    B::render::<AnsiSink<_, Untagged>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard)
        .unwrap();
    println!("");

    let debug = Text::of("Number with Separator:              ").with_component(sep);
    B::render::<AnsiSink<_, _>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    println!("");

    let debug = Text::of("Hex Number:                         ").with_component(b16);
    B::render::<AnsiSink<_, _>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    println!("");

    let debug = Text::of("Hex Number With Separator:          ").with_component(b16_sep);
    B::render::<AnsiSink<_, _>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    println!("");

    let debug = Text::of("Negative Hex Number With Separator: ").with_component(n_b16_sep);
    B::render::<AnsiSink<_, _>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    println!("");
}
