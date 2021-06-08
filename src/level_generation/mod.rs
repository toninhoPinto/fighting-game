use sdl2::rect::Rect;
use tiled::Map;

pub mod generate;

#[derive(Debug)]
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

        let n_cells = map_width * map_height;

        let mut rects = Vec::new();
        for tile in 0..n_cells {
            let x = tile * tile_width % (map_width * tile_width);
            let y = (tile * tile_width / (map_width * tile_width) * tile_width) as i32;
            rects.push(Rect::new(x as i32,y as i32, tile_width, tile_height));
        }
        
        Self {
            start_x: 0,
            width: map_width,
            tiles: rects,
            map,
        }
    }

    pub fn rect_from_index(&self, tile: u32) -> Rect{
        match &self.map.layers[0].tiles {
            tiled::LayerData::Finite(tiles) => {
                let x = tile * self.map.tile_width % (self.map.width * self.map.tile_width);
                let y = tile * self.map.tile_width / (self.map.width * self.map.tile_width);
                
                let tile_texture_index_col = &tiles[y as usize];
                let tile_texture_index = tile_texture_index_col[(x/self.map.tile_width) as usize].gid - 1;

                let texture_width = self.map.tilesets[0].images[0].width as u32;
                let texture_columns = texture_width / self.map.tile_width;

                let tex_x = tile_texture_index % texture_columns * self.map.tile_width;
                let tex_y = tile_texture_index / texture_columns * self.map.tile_width;
                //println!("{:?} {:?},{:?}", tile_texture_index, tex_x, tex_y);

                Rect::new(tex_x as i32, tex_y as i32, self.map.tile_width, self.map.tile_height)
            },
            tiled::LayerData::Infinite(_) => todo!(),
        }
    }
}