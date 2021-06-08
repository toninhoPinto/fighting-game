use sdl2::rect::Rect;
use tiled::Map;

pub mod generate;

pub struct Level {
    pub start_x: i32,
    pub width: u32,
    pub tiles: Vec<Rect>,
    pub map: Map,
}

impl Level {
    pub fn new(map: Map) -> Self {
        let map_width = map.width;
        let map_height = map.height;
        let tile_width = map.tile_width;
        let tile_height = map.tile_height;

        let n_columns = map_width / tile_width;
        let n_rows = map_height / tile_height;
        let n_cells = n_columns * n_rows;

        let mut rects = Vec::new();
        for tile in 0..n_cells {
            let x = tile * tile_width % map_width;
            let y = tile * tile_height % map_height;
            rects.push(Rect::new(x as i32,y as i32,tile_width, tile_height));
        }

        Self {
            start_x: 0,
            width: map_width,
            tiles: rects,
            map,
        }
    }


}