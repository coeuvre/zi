extern crate libc;

use std::io;
use std::io::prelude::*;

pub struct AsciiTerm {
    out: io::Stdout,
}

impl AsciiTerm {
    pub fn new() -> AsciiTerm {
        unsafe {
            use std::mem;
            let mut termios = mem::uninitialized();
            libc::tcgetattr(1, &mut termios);
            termios.c_lflag &= !(libc::ECHO | libc::ICANON);
            libc::tcsetattr(1, libc::TCSAFLUSH, &termios);
        }

        AsciiTerm {
            out: io::stdout(),
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
            Ok(_) => {
                let input = String::from_utf8_lossy(&buf);
                if let Some(ch) = input.chars().next() {
                    match ch {
                        'h' => term.cursor_back(1),
                        'j' => term.cursor_down(1),
                        'k' => term.cursor_up(1),
                        'l' => term.cursor_forward(1),
                        'q' => break 'input,
                        _ => {}
                    }
                }
            }

            Err(e) => panic!(e),
        }

        term.flush();
    }
}
