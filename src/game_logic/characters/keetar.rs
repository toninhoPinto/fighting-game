use crate::{game_logic::{character_factory::CharacterAnimations, game::Game, inputs::{game_inputs::GameAction, input_cycle::AllInputManagement}, projectile::Projectile}, rendering::camera::Camera};

use parry2d::na::Vector2;

use super::{Attack, AttackHeight, AttackType};

//logic only module, no struct

pub fn spawn_light_note(game: &mut Game, id: i32, assets: &CharacterAnimations) {
    spawn_note(game, id, Vector2::new(120.0, 5.0), assets, update_note_light);
}

pub fn update_note_light(p1_inputs: &AllInputManagement, animations: &CharacterAnimations, projectile: &mut Projectile) {
    update_note(p1_inputs, projectile, animations, GameAction::LightPunch)
}

//====================================================================================

pub fn spawn_medium_note(game: &mut Game, id: i32, assets: &CharacterAnimations) {
    spawn_note(game, id, Vector2::new(120.0, 105.0), assets, update_note_medium);
}

pub fn update_note_medium(p1_inputs: &AllInputManagement, animations: &CharacterAnimations, projectile: &mut Projectile) {
    update_note(p1_inputs, projectile, animations, GameAction::MediumPunch)
}

//====================================================================================

pub fn spawn_heavy_note(game: &mut Game, id: i32, assets: &CharacterAnimations) {
    spawn_note(game, id,  Vector2::new(120.0, 205.0), assets, update_note_heavy);
}

pub fn update_note_heavy(p1_inputs: &AllInputManagement, animations: &CharacterAnimations, projectile: &mut Projectile) {
    update_note(p1_inputs, projectile, animations, GameAction::HeavyPunch)
}

//====================================================================================

pub fn update_note(p1_inputs: &AllInputManagement, projectile: &mut Projectile, animations: &CharacterAnimations, keep_alive_action: GameAction) {
    if let Some(&input) = p1_inputs.action_history.back() {
        if projectile.has_reached_target && (input & keep_alive_action as i32) == 0 {
            if let Some(hit_anim) = animations.projectile_animation.get("bloop") {
                projectile.animator.play_once(hit_anim.clone(), 1.0, false);
                projectile.colliders.clear();
                projectile.sprite.set_width(100);
                projectile.sprite.set_height(80);
            }
            projectile.direction = Vector2::new(0.0, 0.0);
            projectile.target_position = None;
            projectile.kill_at_animation_end = true
        }
    }
}

fn spawn_note(game: &mut Game, id: i32, position: Vector2<f64>, assets: &CharacterAnimations,
     update_fn: fn(&AllInputManagement, &CharacterAnimations, &mut Projectile) -> ()) {
    let (player,opponent) = if id == 1 { 
        (&game.player1, &game.player2) 
    } else { 
        (&game.player2,  &game.player1)
    };

    //TODO this should probably be inside character data??
    let projectile_attack = Attack {
        damage: 10,
        stun_on_hit: 2,
        stun_on_block: 6,
        push_back: 100.0,
        attack_height: AttackHeight::ALL,
        attack_type: AttackType::Special,
    };

    let mut projectile = Projectile::new(0, position, projectile_attack);
    
    if player.is_attacking {
        projectile.position += Vector2::new(player.position.x, 0.0);
        projectile.direction = Vector2::new(player.dir_related_of_other.signum() as f64, 0.0);
        projectile.flipped = player.dir_related_of_other > 0;
        projectile.player_owner = player.id;
        projectile.speed = 15;

        projectile.on_update = Some(update_fn);

        let target_pos = Vector2::new(
            opponent.position.x + projectile.direction.x * 100.0,
            projectile.position.y,
        );

        projectile.target_position = Some(target_pos);
        projectile.init(assets.projectile_animation.get("note").unwrap().clone());
        game.projectiles.push(projectile);
    }
}

