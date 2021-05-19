use std::collections::HashSet;

use sdl2::rect::Point;

#[derive(Debug, PartialEq)]
pub enum WorldNodeType {
    Start,
    Boss,
    Level,
    Event,
    Store
}

#[derive(Debug)]
pub struct WorldNode {
    pub node_type: WorldNodeType,
    pub position: Point,
    pub connect_to: HashSet<usize>,
}