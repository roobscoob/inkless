use inkless::{
    buffer::{RenderDispatcher, r#static::StaticRenderBuffer},
    canvas::AmbiguityPolicy,
    gph,
    number::{Number, tag::NumberTag},
    tag::untagged::Untagged,
    text::{Text, TextWithRenderable, tag::TextTag},
    theme::{Theme, ext::RenderableThemeExt},
};
use inkless_term::{sink::AnsiSink, styles::TrueColor, tag::default::Ansi};

const W: usize = 45;
type B<T> = StaticRenderBuffer<T, W>;

struct DemoTheme;

impl Theme<NumberTag> for DemoTheme {
    type Result = Ansi;

    fn translate(from: NumberTag) -> Self::Result {
        Ansi::plain().bg_red()
    }
}

impl Theme<TextTag<Ansi>> for DemoTheme {
    type Result = Ansi;

    fn translate(from: TextTag<Ansi>) -> Self::Result {
        match from {
            TextTag::Ellipsis(e) => Ansi::new().bright_black(),
            TextTag::Segment(s) => s,
        }
    }
}

pub fn main() {
    // let number = Number::new(1024u32);

    // let sep = Number::new(1024u32).with_separator(3, gph!(","));

    // let b16 = Number::new(1024u32).with_base(16).with_prefix("0x");

    // let b16_sep = Number::new(1024u32)
    //     .with_base(16)
    //     .with_separator(2, gph!("_"))
    //     .with_prefix("0x");

    let n_b16_sep = Number::new(-1024i32)
        .with_base(16)
        .with_separator(2, gph!("_"))
        .with_prefix("0x")
        .theme::<DemoTheme>();

    // let debug = Text::of::<Untagged>("Normal number:                      ")
    //     .with_component::<Untagged>(number);

    // B::render::<AnsiSink<_, Untagged>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard)
    //     .unwrap();
    // println!("");

    // let debug = Text::of("Number with Separator:              ").with_component(sep);
    // B::render::<AnsiSink<_, _>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    // println!("");

    // let debug = Text::of("Hex Number:                         ").with_component(b16);
    // B::render::<AnsiSink<_, _>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    // println!("");

    // let debug = Text::of("Hex Number With Separator:          ").with_component(b16_sep);
    // B::render::<AnsiSink<_, _>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    // println!("");

    let debug = Text::of::<Untagged>("Negative Hex Number With Separator: ")
        .with_component::<Untagged>(n_b16_sep);

    B::render::<AnsiSink<_, _>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    println!("");
}
