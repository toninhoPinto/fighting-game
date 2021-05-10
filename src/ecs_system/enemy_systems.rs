use std::collections::HashMap;

use sdl2::{rect::{Point, Rect}, render::Texture};

use crate::{asset_management::animator::Animator, game_logic::{characters::Character, enemy_factory::EnemyAssets}};

use super::{enemy_components::{Health, Position, Renderable}, enemy_manager::EnemyManager};

pub fn update_animations_enemies(enemy_manager: &mut EnemyManager) {
    let zip = enemy_manager.animator_components.iter_mut();
    zip.for_each(|animator| {
        if let Some(animator) = animator {
            animator.update();
        }
    });
}

pub fn render_enemies<'a>(enemy_manager: &EnemyManager, assets: &'a HashMap<&str, EnemyAssets>) -> Vec<(&'a Texture<'a>, Rect, Point, bool)> {
    let zip = enemy_manager
        .animator_components
        .iter()
        .zip(enemy_manager.renderable_components.iter())
        .zip(enemy_manager.positions_components.iter())
        .zip(enemy_manager.character_components.iter());

    let living =
        zip
        .filter_map(|(((animator, renderable), pos), character): (((&Option<Animator>, &Option<Renderable>), &Option<Position>), &Option<Character>)| {
            Some((animator.as_ref()?, renderable.as_ref()?, pos.as_ref()?, character.as_ref()?))
        })
        .map(|(animator, renderable, pos, character): (&Animator, &Renderable, &Position, &Character)| {
            let (tex, rect, offsets) = render_enemy(animator.render(), animator, renderable, assets.get(&character.name as &str).unwrap());
            let pos = Point::new((pos.0.x - offsets.0) as i32, (pos.0.y - offsets.1 )as i32);
            (tex, rect, pos, renderable.flipped)
        });

    let mut enemies_y_sorted = living.collect::<Vec<(&'a Texture<'a>, Rect, Point, bool)>>();
    enemies_y_sorted.sort_by(|a, b| b.2.y.cmp(&a.2.y));
    enemies_y_sorted
}


fn render_enemy<'a>(texture_handle: String, animator: &Animator, renderable: &Renderable, assets: &'a EnemyAssets<'a>) -> (&'a Texture<'a>, Rect, (f64, f64))  {
    let sprite_data = assets.texture_data.get(&texture_handle);
    
    let mut rect = renderable.rect.clone();
    let mut offset = (0f64, 0f64);

    if let Some(sprite_data) = sprite_data {
        rect.resize(sprite_data.width * 2 , sprite_data.height * 2 );

        let pivot_x_offset = if renderable.flipped {(1f64 - sprite_data.pivot_x)* 2.0 * sprite_data.width as f64} else {sprite_data.pivot_x * 2.0 * sprite_data.width as f64};
        let pivot_y_offset = sprite_data.pivot_y * 2.0 * sprite_data.height as f64;

        offset = if let Some(sprite_alignment) = animator.current_animation.as_ref().unwrap().sprite_alignments.get(&animator.sprite_shown) {
            let facing_dir = if renderable.flipped {1} else {-1};
            (pivot_x_offset + facing_dir as f64 * sprite_alignment.pos.x * 2.0, pivot_y_offset + sprite_alignment.pos.y * 2.0)
        } else {
            (pivot_x_offset, pivot_y_offset)
        };

    }
    (assets.textures.get(&texture_handle).unwrap(), rect, offset)
}