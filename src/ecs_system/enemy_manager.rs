use std::rc::Rc;

use parry2d::na::Vector2;
use sdl2::rect::Rect;

use crate::{asset_management::asset_holders::EntityAnimations, collision::collider_manager::ColliderManager, enemy_behaviour::simple_enemy_behaviour::walk_to_player, engine_types::{animation::Animation, animator::Animator}, game_logic::{characters::Character, effects::events_pub_sub::EventsPubSub, factories::enemy_factory::load_enemy, movement_controller::MovementController}};

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
    pub events_components: Vec<Option<EventsPubSub>>,
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
            events_components: Vec::new(),
        }
    }

    fn new_entity(&mut self, 
        health: Option<Health>, 
        behaviour: Option<Behaviour>, 
        player_pos: Vector2<f64>, 
        pos: Option<Position>, 
        character: Option<Character>, 
        animator: Option<Animator>,
        colliders: Option<ColliderManager>,
        events: Option<EventsPubSub>,
        entity_animations: Rc<EntityAnimations>
    ) {
        
        
        if self.health_components.len() < MAX_ENEMIES {
            let movement = match (&character, &pos) {
                (Some(character), Some(pos)) =>  {
                    Some(MovementController::new(&character, pos.0 , player_pos, entity_animations))
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
            self.events_components.push(events);

            let renderable = Renderable {
                flipped: false,
                rect: Rect::new(0,0, 100, 100),
            };
            self.renderable_components.push(Some(renderable));
        }
    }

    pub fn add_enemy(&mut self, player_pos: Vector2<f64>, entity_animations: Rc<EntityAnimations>) {

        let ryu = load_enemy("ryu");

        let mut animator = Animator::new();

        let starting_animation = entity_animations.animations.get("idle").unwrap().clone();
        animator.play(starting_animation, 1.0,false);

        self.new_entity(
            Some(Health(ryu.hp)),
            Some(walk_to_player as Behaviour),
            player_pos,
            Some(Position(player_pos + Vector2::new(500f64, 0f64))),
            Some(ryu),
            Some(animator), 
            Some(ColliderManager::new()),
            Some(EventsPubSub::new()),
            entity_animations
        );

        println!("Spawned entity");
    }
}