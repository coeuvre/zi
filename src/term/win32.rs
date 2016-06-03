extern crate winapi;
extern crate kernel32;

use super::{Event, Key, Term, KEY_MOD_NONE};

use libc;
use std::io;
use std::io::prelude::*;
use std::ffi::CString;
use std::mem;

use self::winapi::winnt::*;
use self::winapi::minwindef::*;
use self::winapi::winbase::*;
use self::winapi::wincon::*;
use self::kernel32::*;

pub struct Win32Term {
    input: HANDLE,
    output: winapi::winnt::HANDLE,
    cursor_pos: (u16, u16),

    orig_input_mode: DWORD,
    orig_cursor_info: CONSOLE_CURSOR_INFO,
    orig_screen_buffer_info: CONSOLE_SCREEN_BUFFER_INFOEX,
}

impl Win32Term {
    pub fn new() -> Win32Term {
        unsafe {
            let input = GetStdHandle(STD_INPUT_HANDLE);
            let output = GetStdHandle(STD_OUTPUT_HANDLE);

            let mut orig_cursor_info = mem::uninitialized();
            GetConsoleCursorInfo(output, &mut orig_cursor_info);

            let mut orig_screen_buffer_info = mem::uninitialized();
            GetConsoleScreenBufferInfoEx(output, &mut orig_screen_buffer_info);

            let mut orig_input_mode = mem::uninitialized();
            GetConsoleMode(input, &mut orig_input_mode);
            SetConsoleMode(input, ENABLE_MOUSE_INPUT | ENABLE_WINDOW_INPUT);
            Win32Term {
                input: input,
                output: output,
                cursor_pos: (0, 0),

                orig_input_mode: orig_input_mode,
                orig_cursor_info: orig_cursor_info,
                orig_screen_buffer_info: orig_screen_buffer_info,
            }
        }
    }
}

pub struct WaitEventIter {
    input: HANDLE,
    count: u32,
    read: u32,
    buf: [INPUT_RECORD; 128],
}

impl Iterator for WaitEventIter {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        if self.read >= self.count {
            self.read = 0;
            unsafe {
                ReadConsoleInputW(self.input, self.buf.as_mut_ptr(), self.buf.len() as u32, &mut self.count);
            }
        }

        while self.read < self.count {
            let record = self.buf[self.read as usize];
            self.read += 1;

            unsafe {
                match record.EventType {
                    KEY_EVENT => {
                        let key_event = record.KeyEvent();
                        if key_event.bKeyDown != 0 {
                            return Some(Event::Key(Key::Unicode(*key_event.AsciiChar() as u8 as char), KEY_MOD_NONE));
                        }
                    }

                    _ => {}
                }
            }
        }

        None
    }
}

impl Drop for Win32Term {
    fn drop(&mut self) {
        unsafe {
            SetConsoleMode(self.input, self.orig_input_mode);
            SetConsoleCursorInfo(self.output, &mut self.orig_cursor_info);
            SetConsoleScreenBufferInfoEx(self.output, &mut self.orig_screen_buffer_info);
        }
    }
}

impl Term for Win32Term {
    fn clear(&mut self) {
        unsafe {
            libc::system(CString::new("cls").unwrap().as_ptr());
        }
    }

    fn move_cursor(&mut self, x: u16, y: u16) {
        unsafe {
            let coord = winapi::wincon::COORD {
                X: x as i16,
                Y: y as i16,
            };
            kernel32::SetConsoleCursorPosition(self.output, coord);
            self.cursor_pos = (x, y);
        }
    }

    fn cursor_up(&mut self, n: u16) {
        let (x, y) = self.cursor_pos;
        if y >= n {
            self.move_cursor(x, y - n);
        }
    }

    fn cursor_down(&mut self, n: u16) {
        let (x, y) = self.cursor_pos;
        self.move_cursor(x, y + n);
    }

    fn cursor_forward(&mut self, n: u16) {
        let (x, y) = self.cursor_pos;
        self.move_cursor(x + n, y);
    }

    fn cursor_back(&mut self, n: u16) {
        let (x, y) = self.cursor_pos;
        if x >= n {
            self.move_cursor(x - n, y);
        }
    }

    fn print(&mut self, ch: char) {
        print!("{}", ch);
        if ch == '\n' {
            self.cursor_pos.1 += 1;
            self.cursor_pos.0 = 0;
        } else if ch == '\r' {
            self.cursor_pos.0 = 0;
        } else {
            self.cursor_pos.0 += 1;
        }
    }

    fn flush(&mut self) {
        io::stdout().flush().unwrap();
    }

    fn wait_events(&mut self) -> Box<Iterator<Item=Event>> {
        Box::new(WaitEventIter {
            input: self.input,
            count: 0,
            read: 0,
            buf: [unsafe { mem::uninitialized() }; 128],
        })
    }
}
