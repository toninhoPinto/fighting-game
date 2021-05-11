use parry2d::na::Vector2;
use sdl2::rect::Rect;

use crate::{asset_management::{animation::Animation, animator::Animator}, collision::collider_manager::ColliderManager, enemy_behaviour::simple_enemy_behaviour::walk_to_player, game_logic::{characters::Character, movement_controller::MovementController}};

use super::enemy_components::{Behaviour, Health, Position, Renderable};


pub const MAX_ENEMIES: usize = 30;
pub struct EnemyManager {
    pub health_components: Vec<Option<Health>>,
    pub positions_components: Vec<Option<Position>>,
    pub character_components: Vec<Option<Character>>,
    pub behaviour_components: Vec<Option<Behaviour>>,
    pub animator_components: Vec<Option<Animator>>,
    pub movement_controller_components: Vec<Option<MovementController>>,
    pub collider_components: Vec<Option<ColliderManager>>,
    pub renderable_components: Vec<Option<Renderable>>,
}

impl EnemyManager {
    pub fn new() -> Self {
        Self {
            health_components: Vec::new(),
            positions_components: Vec::new(),
            character_components: Vec::new(),
            behaviour_components: Vec::new(),
            animator_components: Vec::new(),
            movement_controller_components: Vec::new(),
            collider_components: Vec::new(),
            renderable_components: Vec::new(),
        }
    }

    fn new_entity(&mut self, 
        health: Option<Health>, 
        behaviour: Option<Behaviour>, 
        player_pos: Vector2<f64>, 
        pos: Option<Position>, 
        character: Option<Character>, 
        animator: Option<Animator>,
        colliders: Option<ColliderManager>) {
        
        
        if self.health_components.len() < MAX_ENEMIES {
            let movement = match (&character, &pos) {
                (Some(character), Some(pos)) =>  {
                    Some(MovementController::new(&character, pos.0 , player_pos))
                },
                
                (None, None) | (None, Some(_)) | (Some(_), None) => {None}
            };

            self.movement_controller_components.push(movement);
            self.health_components.push(health);
            self.positions_components.push(pos);
            self.character_components.push(character);
            self.animator_components.push(animator);
            self.behaviour_components.push(behaviour);
            self.collider_components.push(colliders);

            let renderable = Renderable {
                flipped: false,
                rect: Rect::new(0,0, 100, 100),
            };
            self.renderable_components.push(Some(renderable));
        }
    }

    pub fn add_enemy(&mut self, player_pos: Vector2<f64>, starting_animation: Animation) {

        let ryu = Character::new(
            "ryu".to_string(),
            240,
            240,
            100,
            3,
            150.0,
            570.0,
            600.0,
            500.0,
        );

        let mut animator = Animator::new();
        animator.play(starting_animation, 1.0,false);

        self.new_entity(
            Some(Health(ryu.hp)),
            Some(walk_to_player as Behaviour),
            player_pos,
            Some(Position(player_pos + Vector2::new(500f64, 0f64))),
            Some(ryu),
            Some(animator), 
            Some(ColliderManager::new())
        );

        println!("Spawned entity");
    }
}