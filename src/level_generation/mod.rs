use sdl2::rect::Rect;
use tiled::Map;

pub mod generate;

#[derive(Debug)]
pub struct Level {
    pub start_x: i32,
    pub width: u32,
    pub tiles: Vec<Vec<Tile>>,
    pub level_map:  Map,
}

#[derive(Debug)]
pub struct Tile {
    pub rect: Rect,
    pub texture_id: u32,
}

impl Tile {
    pub fn new(rect: Rect, texture_id: u32) -> Self {
        Self {
            rect,
            texture_id,
        }
    }
}

impl Level {
    pub fn new(map: &Map, start_x: i32) -> Self {
        let map_width = map.width;
        let _map_height = map.height;
        let tile_width = map.tile_width;
        let tile_height = map.tile_height;
       
        let mut layers = Vec::new();

        for (_layer_id, layer) in map.layers.iter().enumerate() {
            match &layer.tiles {
                tiled::LayerData::Finite(tiles) => {
                   
                    let mut rects = Vec::new();

                    let mut tile_id = 0u32;
                    for collumn in tiles.iter().enumerate() {
                        for tile in collumn.1.iter().enumerate() {
                            if tile.1.gid != 0  {
                                let x = start_x as u32 + tile_id * tile_width % (map_width * tile_width);
                                let y = (tile_id * tile_width / (map_width * tile_width) * tile_width) as i32;

                                let tile_rect = Rect::new(x as i32,y as i32, tile_width, tile_height);
                                
                                rects.push(Tile::new(tile_rect, tile.1.gid - 1));
                            }
                            tile_id += 1;
                        }
                    }
 
                    layers.push(rects);
                },
                tiled::LayerData::Infinite(_) => todo!(),
            }
        }
        
        Self {
            start_x,
            width: map_width,
            tiles: layers,
            level_map: map.clone(),
        }
    }

    

    pub fn rect_from_index(&self, tile_id: u32, layer_id: usize) -> Rect{
        match &self.level_map.layers[layer_id].tiles {
            tiled::LayerData::Finite(_tiles) => {
                let texture_width = self.level_map.tilesets[0].images[0].width as u32;
                let texture_columns = texture_width / self.level_map.tile_width;

                let tex_x = tile_id % texture_columns * self.level_map.tile_width;
                let tex_y = tile_id / texture_columns * self.level_map.tile_width;

                Rect::new(tex_x as i32, tex_y as i32, self.level_map.tile_width, self.level_map.tile_height)
            },
            tiled::LayerData::Infinite(_) => todo!(),
        }
    }
}