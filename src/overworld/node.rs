use std::collections::HashSet;

use sdl2::rect::Point;

#[derive(Debug, PartialEq)]
pub enum WorldNodeType {
    Start,
    Boss,
    Level(i32),  //Level(difficulty)
    Event(u32),  //Event(id)
    Store,
}

#[derive(Debug)]
pub struct WorldNode {
    pub node_type: WorldNodeType,
    pub position: Point,
    pub connect_to: HashSet<usize>,
}