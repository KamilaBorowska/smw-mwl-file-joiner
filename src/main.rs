extern crate byteorder;
#[macro_use]
extern crate failure;

mod cache;
mod db;
mod entry;
mod level;
mod map16;

use failure::Error;
use std::fs;

use db::Database;
use entry::Entry;

fn main() {
    let mut database = Database::new();
    for dir_entry in fs::read_dir("../entries").unwrap() {
        let entry = Entry::new(&mut database);
        // TODO: find files and add them to an entry
    }
}
