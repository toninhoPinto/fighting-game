use sdl2::rect::{Point, Rect};

pub struct WrappingList {
    pub position: Point,
    pub width: u32,
    pub offset: i32,
    pub rects: Vec<Rect>,
}


impl WrappingList {

    pub fn new(position: Point, width: u32, mut rects: Vec<Rect>, offset: i32) -> Self {
        WrappingList::init(position, width, offset, &mut rects);

        Self {
            position,
            width,
            offset,
            rects,
        } 
    }

    fn init(position: Point, width: u32, offset: i32, rects: &mut Vec<Rect>) {
        let mut row = 0;
        let mut column = 0;
        for i in 0..rects.len() {
            if position.x + (rects[i].width() as i32 + offset) * column as i32 > width as i32 {
                column = 0;
                row += 1;
            }
            let x = position.x + (rects[i].width() as i32 + offset) * column as i32;
            rects[i].x = x;
            rects[i].y = position.y + (offset + rects[i].height() as i32) * row;

            column += 1;
        }
    }

    pub fn update(&mut self, rects: Vec<Rect>) {
        self.rects = rects;
        WrappingList::init(self.position, self.width, self.offset, &mut self.rects); 
    }

    pub fn render(&self) -> Vec<Rect> {
        self.rects.clone()
    }
}