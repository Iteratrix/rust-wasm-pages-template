//! Core domain logic.
//!
//! Keep this crate pure: no I/O, no platform assumptions, wasm-safe
//! dependencies only. Adapter crates (web, cli) stay thin and translate at
//! the edge. This is what lets the same logic ship as a browser app, a CLI,
//! and whatever adapter comes next.

/// What [`summarize`] learned about a byte buffer.
///
/// Placeholder domain type — replace with your real model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary {
    /// Total size in bytes.
    pub len: usize,
    /// Whether the buffer is valid UTF-8.
    pub utf8: bool,
    /// Line count, when the buffer is text.
    pub lines: Option<usize>,
}

/// Summarize a byte buffer.
///
/// Placeholder entry point — replace with your real parsing/processing.
#[must_use]
pub fn summarize(bytes: &[u8]) -> Summary {
    let Ok(text) = core::str::from_utf8(bytes) else {
        return Summary {
            len: bytes.len(),
            utf8: false,
            lines: None,
        };
    };
    Summary {
        len: bytes.len(),
        utf8: true,
        lines: Some(text.lines().count()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_text() {
        let summary = summarize(b"hello\nworld\n");
        assert_eq!(
            summary,
            Summary {
                len: 12,
                utf8: true,
                lines: Some(2)
            }
        );
    }

    #[test]
    fn summarizes_binary() {
        let summary = summarize(&[0xff, 0xfe, 0x00]);
        assert_eq!(
            summary,
            Summary {
                len: 3,
                utf8: false,
                lines: None
            }
        );
    }
}
