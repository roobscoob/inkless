use std::{io::StdoutLock, sync::LazyLock};

use inkless_core::writer::character::CharacterWriter;

use crate::{
    sink::AnsiSink,
    support::{AnsiEnv, AnsiSupport},
};

pub static ANSI_ENV: LazyLock<AnsiEnv<'static>> = LazyLock::new(AnsiEnv::from_env);

impl AnsiEnv<'static> {
    pub fn from_env() -> Self {
        use std::env;
        use std::io;

        // Detect whether stdout is a TTY.
        #[cfg(unix)]
        let is_tty = {
            use std::os::unix::io::AsRawFd;
            unsafe { libc::isatty(io::stdout().as_raw_fd()) != 0 };
        };

        #[cfg(windows)]
        let is_tty = {
            use std::os::windows::io::AsRawHandle;
            use windows_sys::Win32::System::Console::GetConsoleMode;
            let mut mode = 0u32;
            unsafe { GetConsoleMode(io::stdout().as_raw_handle() as _, &mut mode) != 0 }
        };

        // Pull environment variables.
        let term = env::var("TERM").ok();
        let colorterm = env::var("COLORTERM").ok();
        let term_program = env::var("TERM_PROGRAM").ok();
        let vte_version = env::var("VTE_VERSION")
            .ok()
            .and_then(|s| s.parse::<u32>().ok());

        // Leak small strings so we can hand out &'static str references.
        fn leak(s: String) -> &'static str {
            Box::leak(s.into_boxed_str())
        }

        AnsiEnv {
            is_tty,
            term: term.map(leak),
            colorterm: colorterm.map(leak),
            term_program: term_program.map(leak),
            vte_version,
        }
    }
}

/// Adapter: wrap an `io::Write` and expose it as a `fmt::Write`.
pub struct IoWriter<W: std::io::Write>(pub W);

impl<W: std::io::Write> CharacterWriter for IoWriter<W> {
    type Error = std::io::Error;

    fn write_str(&mut self, s: &str) -> Result<(), std::io::Error> {
        self.0.write_all(s.as_bytes())
    }
}

impl AnsiSupport {
    pub fn from_env() -> Self {
        Self::from(*ANSI_ENV)
    }
}

impl<'a> AnsiSink<IoWriter<StdoutLock<'a>>> {
    pub fn stdout() -> Self {
        Self {
            result: Ok(()),
            support: AnsiSupport::from_env(),
            writer: IoWriter(std::io::stdout().lock()),
            last_tag: None,
        }
    }
}
