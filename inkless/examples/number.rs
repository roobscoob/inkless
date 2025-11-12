use inkless::{
    buffer::{RenderDispatcher, r#static::StaticRenderBuffer},
    canvas::AmbiguityPolicy,
    gph,
    number::{Number, tag::NumberTag},
    tag::{Tag, untagged::Untagged},
    text::{Text, TextWithRenderable, tag::TextTag},
    theme::{
        Theme,
        ext::{RenderableThemeExt, ThemedRenderable},
    },
};
use inkless_term::{sink::AnsiSink, tag::default::Ansi};

const W: usize = 45;
type B<T> = StaticRenderBuffer<T, W>;

struct DemoTheme;

impl Theme<NumberTag> for DemoTheme {
    type Result = Ansi;

    fn translate(from: NumberTag) -> Self::Result {
        Ansi::plain().red()
    }
}

impl<A: Tag + Clone + Into<Ansi>, B: Tag + Into<Ansi>> Theme<TextTag<A, B>> for DemoTheme {
    type Result = Ansi;

    fn translate(from: TextTag<A, B>) -> Self::Result {
        match from {
            TextTag::Ellipsis(e) => Ansi::new().bright_black(),
            TextTag::Component(c) => c.into(),
            TextTag::Segment(s) => s.into(),
        }
    }
}

pub fn main() {
    let n_b16_sep: ThemedRenderable<NumberTag, _, _> = Number::new(-1024i32)
        .with_base(16)
        .with_separator(2, gph!("_"))
        .with_prefix("0x")
        .with_theme::<DemoTheme>();

    let debug = Text::of_tagged("Negative Hex Number With Separator: ", Untagged)
        .with_component(n_b16_sep)
        .with_theme::<DemoTheme>();

    B::render(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    println!("");
}
