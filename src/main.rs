#[macro_use]
extern crate bitflags;
extern crate libc;

use term::*;

mod term;

fn main() {
    let mut term = Term::new();
    term.clear();
    term.move_cursor(0, 10);
    for ch in "Hello World\n".chars() {
        term.print(ch);
    }
    term.flush();

    'input: loop {
        for event in term.wait_events() {
            match event {
                Event::Key(key, _) => {
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
                }
            }

            term.flush();
        }
    }
}
