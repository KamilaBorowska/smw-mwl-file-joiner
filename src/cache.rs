use std::collections::HashMap;
use std::hash::Hash;

pub struct Cache<T> {
    cache: HashMap<T, u16>,
    current_number: u16,
}

impl<T: Hash + Eq> Cache<T> {
    pub fn new(starting_number: u16) -> Self {
        Cache {
            cache: HashMap::new(),
            current_number: starting_number,
        }
    }

    pub fn get_number(&mut self, elem: T) -> u16 {
        let current_number = &mut self.current_number;
        *self.cache.entry(elem).or_insert_with(|| {
            let number = *current_number;
            *current_number += 1;
            number
        })
    }
}
