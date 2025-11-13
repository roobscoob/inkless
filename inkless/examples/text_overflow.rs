use inkless::{
    buffer::{RenderDispatcher, r#static::StaticRenderBuffer},
    canvas::AmbiguityPolicy,
    tag::untagged::Untagged,
    text::{Text, TextWithRenderable, WithTagged, overflow::EllipsisPosition, tag::TextTag},
    theme::ext::RenderableThemeExt,
};
use inkless_term::{
    sink::AnsiSink,
    tag::{AnsiTag, default::Ansi},
};

const W: usize = 29;
type B<T> = StaticRenderBuffer<T, W>;

pub fn main() {
    let clip = Text::empty::<Ansi, Untagged>()
        .with_tagged("Hello, world!", Ansi::plain().red())
        .clip();

    let grapheme_wrap = Text::empty::<Ansi, Untagged>()
        .with_tagged("Hello, world!", Ansi::plain().yellow())
        .grapheme_wrap();

    let word_wrap = Text::empty::<Ansi, Untagged>()
        .with_tagged("Hello, world!", Ansi::plain().green())
        .word_wrap();

    let ellipsis_left = Text::empty::<Ansi, Untagged>()
        .with_tagged("Hello, world!", Ansi::plain().blue())
        .ellipsis_at(EllipsisPosition::Left);

    const MAGENTA: &'static dyn AnsiTag = &Ansi::plain().magenta();
    const GREY: &'static dyn AnsiTag = &Ansi::plain().bright_black();
    const PLAIN: &'static dyn AnsiTag = &Ansi::plain();

    let ellipsis_center = Text::empty::<Ansi, Untagged>()
        .with_tagged("Hello, world!", MAGENTA)
        .ellipsis_at(EllipsisPosition::Center);

    let ellipsis_right = Text::empty::<Ansi, Untagged>()
        .with_tagged("Hello, world!", Ansi::plain().cyan())
        .ellipsis_at(EllipsisPosition::Right);

    let yappanese = Text::empty::<Ansi, Untagged>()
        .with_tagged("期待された: . / 実際: .", Ansi::plain().cyan())
        .ellipsis_at(EllipsisPosition::Right);

    let debug = Text::of::<Untagged>("Clip:              ").with_component::<TextTag<_, _>>(clip);
    B::render(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    println!("");

    let debug = Text::of("Grapheme Wrap:     ").with_component(grapheme_wrap);
    B::render(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    println!("");

    let debug = Text::of("Word Wrap:         ").with_component(word_wrap);
    B::render(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    println!("");

    let debug = Text::of("Ellipsis (Left):   ").with_component(ellipsis_left);
    B::render(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    println!("");

    let debug = Text::of_tagged("Ellipsis (Center): ", PLAIN).with_component(ellipsis_center);

    B::render::<AnsiSink<_, &dyn AnsiTag>>(
        AnsiSink::stdout(),
        &debug,
        W,
        AmbiguityPolicy::Standard,
    )
    .unwrap();
    println!("");

    let debug = Text::of("Ellipsis (Right):  ").with_component(ellipsis_right);
    B::render(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    println!("");

    let debug = Text::of("Yappanese:         ").with_component(yappanese);
    B::render(AnsiSink::stdout(), &debug, W, AmbiguityPolicy::Standard).unwrap();
    println!("");
}
