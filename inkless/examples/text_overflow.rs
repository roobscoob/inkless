use inkless::{
    buffer::{RenderDispatcher, r#static::StaticRenderBuffer},
    canvas::AmbiguityPolicy,
    text::{EllipsisMethods, Text, overflow::EllipsisPosition},
};
use inkless_term::{
    sink::AnsiSink,
    tag::{AnsiTag, default::Ansi},
};

const W: usize = 29;
type B<T> = StaticRenderBuffer<T, W>;

pub fn main() {
    let clip = Text::of_tagged("Hello, world!", Ansi::plain().red()).clip();

    let grapheme_wrap = Text::of_tagged("Hello, world!", Ansi::plain().yellow()).grapheme_wrap();

    let word_wrap = Text::of_tagged("Hello, world!", Ansi::plain().green()).word_wrap();

    let ellipsis_left = Text::of_tagged("Hello, world!", Ansi::plain().blue());
    // .ellipsis_tagged_at(EllipsisPosition::Left, Ansi::plain().bright_black());

    const MAGENTA: &'static dyn AnsiTag = &Ansi::plain().magenta();
    const GREY: &'static dyn AnsiTag = &Ansi::plain().bright_black();
    const PLAIN: &'static dyn AnsiTag = &Ansi::plain();

    let ellipsis_center = Text::of_tagged("Hello, world!", MAGENTA);
    // .ellipsis_tagged_at(EllipsisPosition::Center, GREY);

    let ellipsis_right = Text::of_tagged("Hello, world!", Ansi::plain().cyan());
    // .ellipsis_tagged_at(EllipsisPosition::Right, Ansi::plain().bright_black());

    let yappanese = Text::of_tagged("期待された: . / 実際: .", Ansi::plain().cyan());
    // .ellipsis_tagged_at(EllipsisPosition::Right, Ansi::plain().bright_black());

    let debug = Text::of("Clip:              ").with_component(clip);
    B::render::<AnsiSink<_, Ansi>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard)
        .unwrap();
    println!("");

    let debug = Text::of("Grapheme Wrap:     ").with_component(grapheme_wrap);
    B::render::<AnsiSink<_, Ansi>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard)
        .unwrap();
    println!("");

    let debug = Text::of("Word Wrap:         ").with_component(word_wrap);
    B::render::<AnsiSink<_, Ansi>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard)
        .unwrap();
    println!("");

    let debug = Text::of("Ellipsis (Left):   ").with_component(ellipsis_left);
    B::render::<AnsiSink<_, Ansi>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard)
        .unwrap();
    println!("");

    let debug = Text::of_tagged("Ellipsis (Center): ", PLAIN).with_component(ellipsis_center);
    B::render::<AnsiSink<_, _>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    println!("");

    let debug = Text::of("Ellipsis (Right):  ").with_component(ellipsis_right);
    B::render::<AnsiSink<_, Ansi>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard)
        .unwrap();
    println!("");

    let debug = Text::of("Yappanese:         ").with_component(yappanese);
    B::render::<AnsiSink<_, Ansi>>(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard)
        .unwrap();
    println!("");
}
