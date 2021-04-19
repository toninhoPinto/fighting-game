use sdl2::rect::Rect;

use crate::game_logic::characters::player::Player;

#[derive(Debug)]
pub struct Camera {
    pub rect: Rect,
}

impl Camera {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
        }
    }

    pub fn update(&mut self, level_size: i32, player: &Player, player2: &Player) {
        let mut proposed_x =
            (player.position.x + player2.position.x) as i32 / 2 - self.rect.width() as i32 / 2;
        if proposed_x < 0 {
            proposed_x = 0;
        }

        if proposed_x + self.rect.width() as i32 > level_size {
            proposed_x = level_size - self.rect.width() as i32;
        }

        self.rect.set_x(proposed_x);
    }
}
