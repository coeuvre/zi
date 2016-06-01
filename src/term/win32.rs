extern crate winapi;
extern crate kernel32;

use super::{Event, Key, Term, KEY_MOD_NONE};

use libc;
use std::ffi::CString;
use std::mem;

pub struct Win32Term {
    input: winapi::winnt::HANDLE,
    orig_input_mode: winapi::minwindef::DWORD,
}

impl Win32Term {
    pub fn new() -> Win32Term {
        unsafe {
            let mut orig_input_mode = mem::uninitialized();
            let input = kernel32::GetStdHandle(winapi::winbase::STD_INPUT_HANDLE);
            kernel32::GetConsoleMode(input, &mut orig_input_mode);
            kernel32::SetConsoleMode(input, winapi::wincon::ENABLE_MOUSE_INPUT | winapi::wincon::ENABLE_WINDOW_INPUT);
            Win32Term {
                input: input,
                orig_input_mode: orig_input_mode,
            }
        }
    }
}

pub struct WaitEventIter {
    input: winapi::winnt::HANDLE,
    count: u32,
    read: u32,
    buf: [winapi::wincon::INPUT_RECORD; 128],
}

impl Iterator for WaitEventIter {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        if self.read >= self.count {
            self.read = 0;
            unsafe {
                kernel32::ReadConsoleInputW(self.input, self.buf.as_mut_ptr(), self.buf.len() as u32, &mut self.count);
            }
        }

        while self.read < self.count {
            let record = self.buf[self.read as usize];
            self.read += 1;

            unsafe {
                match record.EventType {
                    winapi::wincon::KEY_EVENT => {
                        let key_event = record.KeyEvent();
                        if key_event.bKeyDown != 0 {
                            return Some(Event::Key(Key::Unicode(*key_event.AsciiChar() as u8 as char), KEY_MOD_NONE));
                        }
                    }

                    _ => { unreachable!(); }
                }
            }
        }

        None
    }
}

impl Drop for Win32Term {
    fn drop(&mut self) {
        unsafe {
            kernel32::SetConsoleMode(self.input, self.orig_input_mode);
        }
    }
}

impl Term for Win32Term {
    fn clear(&mut self) {
        unsafe {
            libc::system(CString::new("cls").unwrap().as_ptr());
        }
    }

    fn move_cursor(&mut self, row: u16, col: u16) {
    }

    fn cursor_up(&mut self, n: u16) {
    }

    fn cursor_down(&mut self, n: u16) {
    }

    fn cursor_forward(&mut self, n: u16) {
    }

    fn cursor_back(&mut self, n: u16) {
    }

    fn print(&mut self, ch: char) {
    }

    fn flush(&mut self) {
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
