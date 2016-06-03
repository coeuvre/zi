#[macro_use]
extern crate bitflags;
extern crate libc;

use term::*;
use screen::*;

mod term;
mod screen;

fn screen_to_term(screen: &Screen, term: &mut Term) {
    term.clear();
    for (x, y, cell) in screen.cells() {
        if let Some(ref grapheme) = cell.grapheme {
            term.move_cursor(x, y);
            print!("{}", grapheme.character());
            term.flush();
        }
    }
}

fn main() {
    let mut term = Term::new();
    let mut screen = Screen::new(80, 25);

    screen.cell_at_mut(0, 0).unwrap().grapheme = Some(Grapheme::new("H"));
    screen.cell_at_mut(0, 1).unwrap().grapheme = Some(Grapheme::new("e"));
    screen.cell_at_mut(0, 2).unwrap().grapheme = Some(Grapheme::new("l"));
    screen.cell_at_mut(0, 3).unwrap().grapheme = Some(Grapheme::new("l"));
    screen.cell_at_mut(0, 4).unwrap().grapheme = Some(Grapheme::new("o"));

    screen_to_term(&screen, &mut *term);

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
