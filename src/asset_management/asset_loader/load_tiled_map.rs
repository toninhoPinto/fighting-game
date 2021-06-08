use std::{fs::File, io::BufReader, path::Path};

use tiled::{Map, parse_file};

pub fn load_level(path: String) -> Map {
    let file = &Path::new(&path);
    let map = parse_file(file).unwrap();
    //println!("tilesets {:?}", map.tilesets);
    //println!("layers {:?}", map.layers);
    //println!("object_groups {:?}", map.object_groups);


    map
}