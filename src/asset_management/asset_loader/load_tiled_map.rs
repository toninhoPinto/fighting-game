use std::{path::Path};

use tiled::{Map, parse_file};

pub fn load_level(path: String) -> Map {
    let file = &Path::new(&path);
    let map = parse_file(file).unwrap();

    map
}