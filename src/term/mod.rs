#[cfg(unix)]
mod ascii;
#[cfg(windows)]
mod win32;

#[cfg(unix)]
pub type DefaultTerm = ascii::AsciiTerm;

#[cfg(windows)]
pub type DefaultTerm = win32::Win32Term;

pub trait Term {
    fn clear(&mut self);

    fn move_cursor(&mut self, x: u16, y: u16);

    fn cursor_up(&mut self, n: u16);

    fn cursor_down(&mut self, n: u16);

    fn cursor_forward(&mut self, n: u16);

    fn cursor_back(&mut self, n: u16);

    fn print(&mut self, ch: char);

    fn flush(&mut self);

    fn wait_events(&mut self) -> Box<Iterator<Item=Event>>;
}

impl Term {
    pub fn new() -> Box<Term> {
        Box::new(DefaultTerm::new())
    }
}


pub enum Key {
    Unknown,
    Unicode(char),
}

bitflags! {
    pub flags KeyMod: u8 {
        const KEY_MOD_NONE = 0,
        const KEY_MOD_SHIFT = 1 << 0,
        const KEY_MOD_ALT = 1 << 1,
        const KEY_MOD_CTRL = 1 << 2,
    }
}

pub enum Event {
    Key(Key, KeyMod),
}

