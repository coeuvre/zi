use std::io;

use libc;

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
}

impl Term for AsciiTerm {
    fn clear(&mut self) {
        write!(self.out, "\x1B[2J").unwrap();
    }

    fn move_cursor(&mut self, row: u16, col: u16) {
        // Add 1 because terminal row/col is one-indexed
        write!(self.out, "\x1B[{};{}H", row + 1, col + 1).unwrap();
    }

    fn cursor_up(&mut self, n: u16) {
        write!(self.out, "\x1B[{}A", n).unwrap();
    }

    fn cursor_down(&mut self, n: u16) {
        write!(self.out, "\x1B[{}B", n).unwrap();
    }

    fn cursor_forward(&mut self, n: u16) {
        write!(self.out, "\x1B[{}C", n).unwrap();
    }

    fn cursor_back(&mut self, n: u16) {
        write!(self.out, "\x1B[{}D", n).unwrap();
    }

    fn print(&mut self, ch: char) {
        write!(self.out, "{}", ch).unwrap();
    }

    fn flush(&mut self) {
        self.out.flush().unwrap();
    }
}

