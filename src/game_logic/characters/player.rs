use parry2d::na::Vector2;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

use std::rc::Rc;
use std::{collections::{HashMap, VecDeque}, fmt};

use crate::asset_management::common_assets::CommonAssets;
use crate::game_logic::effects::events_pub_sub::CharacterEventActive;
use crate::{asset_management::asset_holders::{EntityAnimations, EntityAssets, EntityData}, collision::collider_manager::ColliderManager, ecs_system::enemy_components::Health, engine_types::{animator::Animator, sprite_data::SpriteData}, game_logic::{effects::{Effect, ItemEffects, events_pub_sub::{CharacterEvent, EventsPubSub}}, inputs::{game_inputs::GameAction, input_cycle::AllInputManagement}, items::{Item, ItemType}, movement_controller::MovementController}, rendering::camera::Camera};

use super::Character;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum EntityState {
    Idle,
    Walking,
    Jump,
    Jumping,
    Landing,
    Dashing,
    Hurt,
    Knocked,
    KnockedLanding,
    Dropped,
    DroppedLanding,
    Dead,
}
impl fmt::Display for EntityState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct Player {
    pub id: i32,
    pub position: Vector2<f64>,
    pub hp: Health,

    pub controller: MovementController,

    pub animator: Animator,
    pub character_width: f64,
    pub character: Character,

    pub collision_manager: ColliderManager,

    pub events: EventsPubSub,
    pub items: Vec<String>,
    pub active_item_key: Option<String>,
    pub active_item: Option<(CharacterEventActive, Effect)>
}

impl Player {
    pub fn new(id: i32, character: Character, spawn_position: Point, animations: Rc<EntityAnimations>) -> Self {
        let pos_as_vec = Vector2::new(spawn_position.x as f64, spawn_position.y  as f64);
        Self {
            id,
            hp: Health(character.hp),
            position: pos_as_vec,

            controller: MovementController::new(&character, pos_as_vec, pos_as_vec, animations),

            animator: Animator::new(),

            character_width: 50.0,
            character,

            collision_manager: ColliderManager::new(),

            events: EventsPubSub::new(),
            items: Vec::new(),

            active_item_key: None,
            active_item: None,
        }
    }

    pub fn equip_item(&mut self, item: &mut Item, hash_effects: &HashMap<i32, ItemEffects>){
        if item.item_type != ItemType::ActivePart {
            self.items.push(item.asset_id.clone());
        } else {
            self.active_item_key = Some(item.asset_id.clone());
        }
        for effect in item.effects.iter_mut() {
            if let Some(apply_effect) = hash_effects.get(&effect.effect_id) {
                apply_effect(self, effect);
            }
        }
    }

    pub fn attack(&mut self, _character_data: &EntityData, attack_animation: String) {
        self.controller.is_attacking = true;
        self.controller.combo_counter += 1;

        self.collision_manager.collisions_detected.clear();
        self.controller.has_hit = false;

        if let Some(attack_anim) = self.controller.animations.animations.get(&attack_animation) { 
            self.animator.play_animation(attack_anim.clone(),1.0, false, true, true);
        }

        if let Some(_) = self.animator.current_animation.as_ref().unwrap().collider_animation {
            self.collision_manager.init_colliders(&self.animator);
        }
    }

    pub fn jump(&mut self) {
        self.controller.jump(&mut self.animator);
    }


    pub fn update(
        &mut self,
        camera: &mut Camera,
        dt: f64,
        character_width: i32,
        general_assets: &CommonAssets
    ) {
       self.controller.update(&mut self.position, &self.character, &mut self.animator, camera, dt, character_width, general_assets);
    }

    pub fn state_update(&mut self, sprite_data: &HashMap<String, SpriteData>) {
        if self.animator.is_finished {
            self.collision_manager.collisions_detected.clear();
            self.controller.has_hit = false;
        }

        self.controller.state_update(&mut self.animator, true);

        self.collision_manager.update_colliders(self.controller.facing_dir > 0, 
            self.position,  &self.animator, sprite_data)
    }
    
    pub fn render<'a>(&'a mut self, assets: &'a EntityAssets<'a>) -> (&'a Texture<'a>, Rect, Point, bool, i32) {
        let key = &self.animator.render();

        let sprite_data = assets.texture_data.get(key);
        
        let rect = &mut self.character.sprite;
        let mut offset = (0f64, 0f64);

        if let Some(sprite_data) = sprite_data {
            rect.resize(sprite_data.width * 2 , sprite_data.height * 2 );

            let pivot_x_offset = if self.controller.facing_dir >= 0 {(1f64 - sprite_data.pivot_x)* 2.0 * sprite_data.width as f64} else {sprite_data.pivot_x * 2.0 * sprite_data.width as f64};
            let pivot_y_offset = sprite_data.pivot_y * 2.0 * sprite_data.height as f64;

            offset = if let Some(sprite_alignment) = self.animator.current_animation.as_ref().unwrap().sprite_alignments.get(&self.animator.sprite_shown) {
                (pivot_x_offset + self.controller.facing_dir as f64 * sprite_alignment.pos.x * 2.0, pivot_y_offset + sprite_alignment.pos.y * 2.0)
            } else {
                (pivot_x_offset, pivot_y_offset)
            };

        }
        
        let pos_to_render = Point::new((self.position.x - offset.0) as i32, (self.position.y - offset.1 )as i32 );
        (assets.textures.get(key).unwrap(), rect.clone(), pos_to_render, self.controller.facing_dir >= 0 , self.controller.ground_height)
    }
}