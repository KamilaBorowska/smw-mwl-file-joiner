use std::collections::{HashMap, HashSet};

pub struct Optimizer {
    next: u16,
    tiles: HashMap<Tile, u16>,
    rectangles: HashMap<Rectangle, u16>,
    big_tiles: HashSet<u16>,
}

impl Optimizer {
    pub fn new() -> Self {
        Optimizer {
            next: 0x400,
            tiles: HashMap::new(),
            rectangles: HashMap::new(),
            big_tiles: HashSet::new(),
        }
    }

    pub fn assign_tile(&mut self, tile: Tile) -> u16 {
        let Optimizer {
            ref mut next,
            ref mut tiles,
            ref mut big_tiles,
            ..
        } = *self;
        *tiles.entry(tile).or_insert_with(|| {
            while big_tiles.remove(next) {
                *next += 1;
            }
            let tile_number = *next;
            *next += 1;
            tile_number
        })
    }

    pub fn assign_rectangle(&mut self, rectangle: Rectangle) -> u16 {
        if let Some(&position) = self.rectangles.get(&rectangle) {
            return position;
        }
        let next = (self.next..)
            .find(|&next| self.is_rectangle_allowed(&rectangle, next))
            .unwrap();
        {
            let mut tiles_iter = rectangle.tiles.iter();
            for y in 0..rectangle.height() {
                for x in 0..rectangle.width {
                    let position = next + x as u16 + y as u16 * 0x10;
                    self.tiles.insert(*tiles_iter.next().unwrap(), position);
                    self.big_tiles.insert(position);
                }
            }
        }
        self.rectangles.insert(rectangle, next);
        next
    }

    fn is_rectangle_allowed(&self, rectangle: &Rectangle, next: u16) -> bool {
        if next % 0x10 + rectangle.width as u16 > 0x10 {
            return false;
        }
        for y in 0..rectangle.height() {
            for x in 0..rectangle.width {
                if self.big_tiles.contains(
                    &(next + x as u16 + y as u16 * 0x10),
                )
                {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Tile {
    tile: [u8; 8],
    behaviour: u16,
}

impl Tile {
    pub fn new(tile: [u8; 8], behaviour: u16) -> Tile {
        Tile { tile, behaviour }
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct Rectangle {
    tiles: Vec<Tile>,
    width: u8,
}

impl Rectangle {
    pub fn new(tiles: Vec<Tile>, width: u8) -> Rectangle {
        Rectangle { tiles, width }
    }

    fn height(&self) -> u8 {
        self.tiles.len() as u8 / self.width
    }
}

#[cfg(test)]
mod test {
    use super::{Optimizer, Tile, Rectangle};

    #[test]
    fn test_optimizer() {
        let mut optimizer = Optimizer::new();
        assert_eq!(
            optimizer.assign_rectangle(Rectangle::new(vec![Tile::new([0; 8], 0); 16], 4)),
            0x400
        );
        assert_eq!(
            optimizer.assign_rectangle(Rectangle::new(vec![Tile::new([1; 8], 0); 11], 11)),
            0x404
        );
        assert_eq!(optimizer.assign_tile(Tile::new([2; 8], 0)), 0x40F);
        assert_eq!(optimizer.assign_tile(Tile::new([3; 8], 0)), 0x414);
        assert_eq!(
            optimizer.assign_rectangle(Rectangle::new(vec![Tile::new([6; 8], 0); 13], 13)),
            0x440
        );
        assert_eq!(
            optimizer.assign_rectangle(Rectangle::new(vec![Tile::new([7; 8], 0); 12], 12)),
            0x424
        );
        assert_eq!(optimizer.assign_tile(Tile::new([4; 8], 0)), 0x415);
        assert_eq!(optimizer.assign_tile(Tile::new([1; 8], 0)), 0x40E);
        assert_eq!(
            optimizer.assign_rectangle(Rectangle::new(vec![Tile::new([0; 8], 0); 16], 4)),
            0x400
        );
        assert_eq!(
            optimizer.assign_rectangle(Rectangle::new(vec![Tile::new([0; 8], 0); 4], 2)),
            0x43D
        );
    }
}
