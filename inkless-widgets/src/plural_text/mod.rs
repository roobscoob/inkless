use crate::number::Number;

struct PluralText<T> {
    rule: PluralTextRule,
    value: Number<T>,
}

enum PluralTextRule {
    Replace(&'static str),
    PrefixNumeral(&'static str),
    PostfixNumeral(&'static str),
    InfixNumeral(&'static str, &'static str),
}

plural! {
    UnreadMessages {
        0 => "no unread messages",
        1 => "{} unread message",
        _ => "{} unread messages",
    }
}

plural! {
    UnreadMessages {
        (count % 10 is 1) && (count % 100 != 11) => "{} файл",
        (count % 10 in 2..=4) && !(count % 100 in 12..=14) => "{} файла",
        _ => "{} файлов",
    }
}

text!(|unread| [MyTag::OtherThing] "you have " ([Separator => MyTag::Thing] unread_messages(unread)) ".");
