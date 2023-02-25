use crate::synchronization::interface::Mutex;
use crate::{null_console, synchronization::NullLock};

pub mod interface {
    use core::fmt;

    pub trait Write {
        /// Write a single character
        fn write_char(&self, c: char);

        /// Write a Rust format string
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;

        /// Block until the last character has been physically put on the TX wire
        fn flush(&self);
    }

    pub trait Read {
        /// Read a single character
        fn read_char(&self) -> char {
            ' '
        }

        /// Clear RX buffers, if any.
        fn clear_rx(&self);
    }

    pub trait Statistics {
        /// Return the number of characters written.
        fn chars_written(&self) -> usize {
            0
        }

        /// Return the number of character read.
        fn chars_read(&self) -> usize {
            0
        }
    }

    /// Trait alias for a full-fledged console.
    pub trait All: Write + Read + Statistics {}
}

static CURRENT_CONSOLE: NullLock<&'static (dyn interface::All + Sync)> =
    NullLock::new(&null_console::NULL_CONSOLE);

/// Register a new console.
pub fn register_console(new_console: &'static (dyn interface::All + Sync)) {
    CURRENT_CONSOLE.lock(|con| *con = new_console);
}

/// Return a reference to the currently registered console.
///
/// This is the global console used by all printing macros.
pub fn console() -> &'static dyn interface::All {
    CURRENT_CONSOLE.lock(|con| *con)
}
