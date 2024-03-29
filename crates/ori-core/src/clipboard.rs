//! Types pertaining to the clipboard.

use crate::log::warn_internal;

/// A clipboard.
pub trait Clipboard {
    /// Returns the contents of the clipboard.
    fn get(&mut self) -> String;

    /// Sets the contents of the clipboard.
    fn set(&mut self, contents: String);
}

struct DummyClipboard;

impl Clipboard for DummyClipboard {
    fn get(&mut self) -> String {
        warn_internal!("Clipboard context not set!");
        String::new()
    }

    fn set(&mut self, _contents: String) {
        warn_internal!("Clipboard context not set!");
    }
}

/// The clipboard context.
pub struct ClipboardContext {
    provider: Box<dyn Clipboard>,
}

impl Default for ClipboardContext {
    fn default() -> Self {
        Self::dummy()
    }
}

impl ClipboardContext {
    /// Creates a dummy clipboard, that does nothing.
    pub fn dummy() -> Self {
        Self {
            provider: Box::new(DummyClipboard),
        }
    }

    /// Creates a new clipboard.
    pub fn new(provider: impl Clipboard + 'static) -> Self {
        Self {
            provider: Box::new(provider),
        }
    }

    /// Returns the contents of the clipboard.
    pub fn get(&mut self) -> String {
        self.provider.get()
    }

    /// Sets the contents of the clipboard.
    pub fn set(&mut self, contents: String) {
        self.provider.set(contents);
    }
}

impl Clipboard for ClipboardContext {
    fn get(&mut self) -> String {
        self.get()
    }

    fn set(&mut self, contents: String) {
        self.set(contents);
    }
}
