use crossterm::event;
use std::fmt;

/// Represents an key.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Key {
    /// Both Enter (or Return) and numpad Enter
    Enter,
    /// Tabulation key
    Tab,
    /// Backspace key
    Backspace,
    /// Escape key
    Esc,

    /// Left arrow
    Left,
    /// Right arrow
    Right,
    /// Up arrow
    Up,
    /// Down arrow
    Down,
    /// Delete key
    Delete,
    /// Home key
    Home,
    /// End key
    End,
    /// Page Up key
    PageUp,
    /// Page Down key
    PageDown,
    Char(char),
    Ctrl(char),
    Unknown,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Key::Left | Key::Right | Key::Up | Key::Down => write!(f, "<{:?} Arrow Key>", self),
            Key::Ctrl(' ') => write!(f, "<Ctrl+Space>"),
            Key::Char(' ') => write!(f, "<Space>"),
            Key::Ctrl(c) => write!(f, "<Ctrl+{}>", c),
            Key::Char(c) => write!(f, "{}", c),
            Key::Enter
            | Key::Tab
            | Key::Backspace
            | Key::Esc
            | Key::Delete
            | Key::Home
            | Key::End
            | Key::PageUp
            | Key::PageDown => write!(f, "<{:?}>", self),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<event::KeyEvent> for Key {
    fn from(key_event: event::KeyEvent) -> Self {
        match key_event {

            event::KeyEvent {
              code: event::KeyCode::Esc,
              ..
            } => Key::Esc,

            event::KeyEvent {
              code: event::KeyCode::Backspace,
              ..
            } => Key::Backspace,

            event::KeyEvent {
              code: event::KeyCode::Left,
              ..
            } => Key::Left,

            event::KeyEvent {
              code: event::KeyCode::Right,
              ..
            } => Key::Right,

            event::KeyEvent {
              code: event::KeyCode::Up,
              ..
            } => Key::Up,

            event::KeyEvent {
              code: event::KeyCode::Down,
              ..
            } => Key::Down,

            event::KeyEvent {
              code: event::KeyCode::Home,
              ..
            } => Key::Home,

            event::KeyEvent {
              code: event::KeyCode::End,
              ..
            } => Key::End,

            event::KeyEvent {
              code: event::KeyCode::PageUp,
              ..
            } => Key::PageUp,

            event::KeyEvent {
              code: event::KeyCode::PageDown,
              ..
            } => Key::PageDown,

            event::KeyEvent {
              code: event::KeyCode::Delete,
              ..
            } => Key::Delete,

            event::KeyEvent {
              code: event::KeyCode::Insert,
              ..
            } => Key::Enter,

            event::KeyEvent {
              code: event::KeyCode::Tab,
              ..
            } => Key::Tab,

            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => Key::Ctrl(c),

            event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            } => Key::Char(c),

            event::KeyEvent {
                code: event::KeyCode::Enter,
                ..
            } => Key::Enter,
            _ => Key::Unknown,
        }
    }
}
