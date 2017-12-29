use db::Database;
use map16::{Rectangle, Tile};

use byteorder::{ByteOrder, LE};
use failure::Error;
use std::collections::HashMap;
use std::path::Path;

pub struct Entry<'a> {
    db: &'a mut Database,
    exgfx: HashMap<u16, u16>,
    tiles: HashMap<u16, Tile>,
}

impl<'a> Entry<'a> {
    pub fn new(db: &mut Database) -> Entry {
        Entry {
            db,
            exgfx: HashMap::new(),
            tiles: HashMap::new(),
        }
    }
}
