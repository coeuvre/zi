pub struct Screen {
    w: u16,
    h: u16,
    cells: Vec<Cell>,
}

impl Screen {
    pub fn new(w: u16, h: u16) -> Screen {
        Screen {
            w: w,
            h: h,
            cells: vec![Cell::new(); (w * h) as usize],
        }
    }

    pub fn cells(&self) -> CellIter {
        CellIter {
            screen: self,
            index: 0,
        }
    }

    pub fn cell_at_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        self.cells.get_mut((y * self.w + x) as usize)
    }
}

pub struct CellIter<'a> {
    screen: &'a Screen,
    index: u16,
}

impl<'a> Iterator for CellIter<'a> {
    type Item = (u16, u16, &'a Cell);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cell) = self.screen.cells.get(self.index as usize) {
            let x = self.index % self.screen.w;
            let y = self.index / self.screen.w;
            self.index += 1;
            Some((x, y, cell))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Cell {
    pub attr: Attr,
    pub grapheme: Option<Grapheme>,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            attr: Attr::None,
            grapheme: None,
        }
    }
}

#[derive(Clone)]
pub enum Attr {
    None
}

#[derive(Clone)]
pub struct Grapheme {
    character: String,
}

impl Grapheme {
    pub fn new(ch: &str) -> Grapheme {
        Grapheme {
            character: ch.to_string(),
        }
    }

    pub fn character(&self) -> &str {
        &self.character
    }
}
