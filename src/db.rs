use cache::Cache;
use map16::{Optimizer, Rectangle, Tile};

pub struct Database {
    fg: Optimizer,
    bg: Optimizer,
    exgfx: Cache<Vec<u8>>,
    current_secondary: u16,
    current_regular: u16,
    current_sublevel: u16,
}

impl Database {
    pub fn new() -> Database {
        Database {
            fg: Optimizer::new(),
            bg: Optimizer::new(),
            exgfx: Cache::new(0x80),
            current_secondary: 1,
            current_regular: 1,
            current_sublevel: 0x25,
        }
    }

    pub fn exgfx_number(&mut self, exgfx: Vec<u8>) -> u16 {
        self.exgfx.get_number(exgfx)
    }

    pub fn tile_number(&mut self, tile: Tile) -> u16 {
        self.fg.assign_tile(tile)
    }

    pub fn tile_number_for_rectangle(&mut self, rectangle: Rectangle) -> u16 {
        self.fg.assign_rectangle(rectangle)
    }

    fn next_regular(&mut self) {
        self.current_regular += match self.current_regular % 0x200 {
            0x24 => 0x101 - 0x24,
            0x13B => 0x201 - 0x13B,
            _ => 1,
        };
    }

    fn next_sublevel(&mut self) {
        self.current_sublevel += 1 +
            match self.current_regular % 0x200 {
                0xFF => 0x3C,
                0x1FF => 0x25,
                _ => 0,
            };
    }
}
