use crate::utils::StrIgnoreCaseExt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnsiColorSupport {
    /// No color support; only default foreground/background.
    None,
    /// Basic 8-color palette (`30–37`, `40–47`).
    Ansi8,
    /// 16-color palette (`30–37`, `40–47`, plus `90–97`, `100–107`).
    Ansi16,
    /// 256-color indexed palette (`38;5;n`, `48;5;n`).
    Ansi256,
    /// True 24-bit RGB colors (`38;2;r;g;b`, `48;2;r;g;b`).
    TrueColor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnsiUnderlineColorSupport {
    /// Underline color cannot be controlled separately from the foreground.
    None,
    /// Underline can use 256-color indexing (`58;5;n`).
    Ansi256,
    /// Underline can use full truecolor (`58;2;r;g;b`).
    TrueColor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnsiSupport {
    pub color: AnsiColorSupport,
    pub underline: AnsiUnderlineColorSupport,
    pub hyperlinks: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct AnsiEnv<'a> {
    /// Whether the output is actually a terminal / TTY.
    pub is_tty: bool,
    /// TERM, if known (e.g., "xterm-256color").
    pub term: Option<&'a str>,
    /// COLORTERM, if known (e.g., "truecolor").
    pub colorterm: Option<&'a str>,
    /// TERM_PROGRAM, if known (e.g., "WezTerm", "iTerm.app").
    pub term_program: Option<&'a str>,
    /// Parsed VTE_VERSION (e.g., 6003 for 0.60.3) if present.
    pub vte_version: Option<u32>,
}

impl AnsiSupport {
    pub fn none() -> Self {
        Self {
            color: AnsiColorSupport::None,
            underline: AnsiUnderlineColorSupport::None,
            hyperlinks: false,
        }
    }

    pub fn with_color(self, color: AnsiColorSupport) -> Self {
        Self {
            color,
            underline: self.underline,
            hyperlinks: self.hyperlinks,
        }
    }

    pub fn with_underline_color(self, color: AnsiUnderlineColorSupport) -> Self {
        Self {
            color: self.color,
            underline: color,
            hyperlinks: self.hyperlinks,
        }
    }

    pub fn with_hyperlinks(self, hyperlinks: bool) -> Self {
        Self {
            color: self.color,
            underline: self.underline,
            hyperlinks,
        }
    }

    pub fn from_env<'a>(env: AnsiEnv<'a>) -> AnsiSupport {
        if !env.is_tty || env.term.is_some_and(|v| v == "dumb") {
            return AnsiSupport::none();
        }

        let color = AnsiColorSupport::from_env(env);
        let underline = AnsiUnderlineColorSupport::from_env(env, color);
        let hyperlinks = detect_hyperlink_support(env);

        Self {
            color,
            hyperlinks,
            underline,
        }
    }
}

impl AnsiColorSupport {
    pub fn from_env<'a>(env: AnsiEnv<'a>) -> AnsiColorSupport {
        if env
            .colorterm
            .is_some_and(|v| v.eq_ignore_ascii_case("truecolor") || v.eq_ignore_ascii_case("24bit"))
        {
            return AnsiColorSupport::TrueColor;
        }

        if env.colorterm.is_some_and(|v| v.contains("256")) {
            return AnsiColorSupport::Ansi256;
        }

        if env.term.is_some_and(|v| {
            v.eq_ignore_ascii_case("-direct") || v.eq_ignore_ascii_case("-truecolor")
        }) {
            return AnsiColorSupport::TrueColor;
        }

        if env
            .term
            .is_some_and(|v| v.ends_with_ignore_ascii_case("-256color"))
        {
            return AnsiColorSupport::Ansi256;
        }

        if env.term.is_some_and(|v| {
            v.eq_ignore_ascii_case("xterm-kitty")
                || v.eq_ignore_ascii_case("alacritty")
                || v.eq_ignore_ascii_case("foot-direct")
                || v.eq_ignore_ascii_case("wezterm")
                || v.eq_ignore_ascii_case("kitty")
        }) {
            return AnsiColorSupport::TrueColor;
        }

        if env.term.is_some_and(|v| {
            v.eq_ignore_ascii_case("linux")
                || v.eq_ignore_ascii_case("xterm")
                || v.eq_ignore_ascii_case("screen")
                || v.eq_ignore_ascii_case("tmux")
                || v.eq_ignore_ascii_case("rxvt")
                || v.eq_ignore_ascii_case("color")
        }) {
            return AnsiColorSupport::Ansi16;
        }

        AnsiColorSupport::Ansi8
    }
}

impl AnsiUnderlineColorSupport {
    pub fn from_env<'a>(env: AnsiEnv<'a>, color: AnsiColorSupport) -> Self {
        if matches!(color, AnsiColorSupport::None) {
            return AnsiUnderlineColorSupport::None;
        }

        if let Some(tp) = env.term_program {
            if is_truecolor_underline_terminal(tp) {
                return match color {
                    AnsiColorSupport::TrueColor => AnsiUnderlineColorSupport::TrueColor,
                    AnsiColorSupport::Ansi256 => AnsiUnderlineColorSupport::Ansi256,
                    _ => AnsiUnderlineColorSupport::None,
                };
            }

            if is_vte_program(tp) {
                if let Some(vte) = env.vte_version {
                    if vte >= 5000 {
                        return match color {
                            AnsiColorSupport::TrueColor => AnsiUnderlineColorSupport::TrueColor,
                            AnsiColorSupport::Ansi256 => AnsiUnderlineColorSupport::Ansi256,
                            _ => AnsiUnderlineColorSupport::Ansi256,
                        };
                    } else if vte >= 3800 {
                        if matches!(
                            color,
                            AnsiColorSupport::Ansi256 | AnsiColorSupport::TrueColor
                        ) {
                            return AnsiUnderlineColorSupport::Ansi256;
                        }
                    }
                }
            }
        } else if env.vte_version.is_some() {
            if matches!(color, AnsiColorSupport::TrueColor) {
                return AnsiUnderlineColorSupport::TrueColor;
            } else if matches!(color, AnsiColorSupport::Ansi256) {
                return AnsiUnderlineColorSupport::Ansi256;
            }
        }

        AnsiUnderlineColorSupport::None
    }
}

fn detect_hyperlink_support(env: AnsiEnv<'_>) -> bool {
    if !env.is_tty {
        return false;
    }

    // Known terminals that implement OSC 8 hyperlinks.
    if let Some(tp) = env.term_program {
        if is_hyperlink_terminal(tp) {
            return true;
        }

        // VTE-based terminals: hyperlinks are supported in reasonably recent versions.
        if is_vte_program(tp) {
            if let Some(vte) = env.vte_version {
                if vte >= 3600 {
                    return true;
                }
            } else {
                // TERM_PROGRAM advertises a VTE terminal but no version: be slightly optimistic.
                return true;
            }
        }
    } else if env.vte_version.is_some() {
        // No TERM_PROGRAM, but VTE_VERSION exists => probably GNOME-ish; hyperlinks are common.
        return true;
    }

    false
}

fn is_vte_program(tp: &str) -> bool {
    tp.eq_ignore_ascii_case("gnome-terminal")
        || tp.eq_ignore_ascii_case("gnome-terminal-server")
        || tp.eq_ignore_ascii_case("tilix")
        || tp.eq_ignore_ascii_case("io.elementary.terminal")
        || tp.eq_ignore_ascii_case("pantheon-terminal")
        || tp.eq_ignore_ascii_case("xfce-terminal")
        || tp.eq_ignore_ascii_case("kgx")
        || tp.eq_ignore_ascii_case("io.mate.terminal")
}

fn is_truecolor_underline_terminal(tp: &str) -> bool {
    tp.eq_ignore_ascii_case("wezterm")
        || tp.eq_ignore_ascii_case("iterm.app")
        || tp.eq_ignore_ascii_case("kitty")
        || tp.eq_ignore_ascii_case("alacritty")
        || tp.eq_ignore_ascii_case("ghostty")
        || tp.eq_ignore_ascii_case("foot")
        || tp.eq_ignore_ascii_case("contour")
        || tp.eq_ignore_ascii_case("windows_terminal")
        || tp.eq_ignore_ascii_case("tabby")
        || tp.eq_ignore_ascii_case("hyper")
        || tp.eq_ignore_ascii_case("warpterminal")
}

fn is_hyperlink_terminal(tp: &str) -> bool {
    tp.eq_ignore_ascii_case("wezterm")
        || tp.eq_ignore_ascii_case("iterm.app")
        || tp.eq_ignore_ascii_case("kitty")
        || tp.eq_ignore_ascii_case("alacritty")
        || tp.eq_ignore_ascii_case("ghostty")
        || tp.eq_ignore_ascii_case("foot")
        || tp.eq_ignore_ascii_case("contour")
        || tp.eq_ignore_ascii_case("windows_terminal")
        || tp.eq_ignore_ascii_case("tabby")
        || tp.eq_ignore_ascii_case("hyper")
        || tp.eq_ignore_ascii_case("warpterminal")
        || tp.eq_ignore_ascii_case("vscode")
        || tp.eq_ignore_ascii_case("visual studio code")
        || tp.eq_ignore_ascii_case("jetbrains")
        || is_vte_program(tp)
}
