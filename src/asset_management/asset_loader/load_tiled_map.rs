use std::{fs::File, io::BufReader, path::Path};

use tiled::{Map, parse};

pub fn load_level(path: String) -> Map {
    let file = File::open(&Path::new(&path)).unwrap();
    println!("Opened file");
    let reader = BufReader::new(file);
    let map = parse(reader).unwrap();
    println!("{:?}", map);
    println!("{:?}", map.get_tileset_by_gid(22));

    map
}