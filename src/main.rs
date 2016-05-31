#[macro_use]
extern crate bitflags;
extern crate libc;

use std::io;
use std::io::prelude::*;

struct Termios {
    fd: libc::c_int,
    orig: libc::termios,
}

impl Termios {
    pub fn new(fd: libc::c_int) -> Termios {
        unsafe {
            use std::mem;
            let mut termios = mem::uninitialized();
            libc::tcgetattr(fd, &mut termios);
            let orig = termios.clone();

            termios.c_lflag &= !(libc::ECHO | libc::ECHONL | libc::ICANON | libc::ISIG | libc::IEXTEN);
            libc::tcsetattr(fd, libc::TCSAFLUSH, &termios);

            Termios {
                fd: fd,
                orig: orig,
            }
        }
    }
}

impl Drop for Termios {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(self.fd, libc::TCSAFLUSH, &self.orig);
        }
    }
}

pub struct AsciiTerm {
    out: io::Stdout,
    _termios: Termios,
}

impl AsciiTerm {
    pub fn new() -> AsciiTerm {
        AsciiTerm {
            out: io::stdout(),
            _termios: Termios::new(1),
        }
    }

    pub fn clear(&mut self) {
        write!(self.out, "\x1B[2J").unwrap();
    }

    pub fn move_cursor(&mut self, row: u16, col: u16) {
        // Add 1 because terminal row/col is one-indexed
        write!(self.out, "\x1B[{};{}H", row + 1, col + 1).unwrap();
    }

    pub fn cursor_up(&mut self, n: u16) {
        write!(self.out, "\x1B[{}A", n).unwrap();
    }

    pub fn cursor_down(&mut self, n: u16) {
        write!(self.out, "\x1B[{}B", n).unwrap();
    }

    pub fn cursor_forward(&mut self, n: u16) {
        write!(self.out, "\x1B[{}C", n).unwrap();
    }

    pub fn cursor_back(&mut self, n: u16) {
        write!(self.out, "\x1B[{}D", n).unwrap();
    }

    pub fn print(&mut self, ch: char) {
        write!(self.out, "{}", ch).unwrap();
    }

    pub fn flush(&mut self) {
        self.out.flush().unwrap();
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

fn main() {
    let mut term = AsciiTerm::new();
    term.clear();
    term.move_cursor(0, 10);
    for ch in "Hello World\n".chars() {
        term.print(ch);
    }
    term.flush();

    let mut buf = [0; 1024];

    'input: loop {
        match io::stdin().read(&mut buf) {
            Ok(n) => {
                let key = {
                    if n == 1 {
                        let ch = buf[0];
                        use std::ascii::AsciiExt;
                        if ch.is_ascii() {
                            Key::Unicode(ch as char)
                        } else {
                            Key::Unknown
                        }
                    } else {
                        Key::Unknown
                    }
                };

                match key {
                    Key::Unicode(codepoint) => {
                        match codepoint {
                            'h' => term.cursor_back(1),
                            'j' => term.cursor_down(1),
                            'k' => term.cursor_up(1),
                            'l' => term.cursor_forward(1),
                            'q' => break 'input,
                            _ => { term.print(codepoint); }
                        }
                    }

                    _ => {}
                }

                /*
                for i in 0..n {
                    print!("0x{:x}", buf[i]);
                }

                let input = String::from_utf8_lossy(&buf);
                if let Some(ch) = input.chars().next() {
                }
                */
            }

            Err(e) => println!("Err: {}", e),
        }

        term.flush();
    }
}
