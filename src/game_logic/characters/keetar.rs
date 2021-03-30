use crate::game_logic::character_factory::CharacterAssets;
use crate::game_logic::projectile::Projectile;

//logic only module, no struct

/*

            let player_current_animation = player1.animator.current_animation.unwrap();
            let p1_curr_anim = player_current_animation.length;
            if (player1.animator.animation_index as f32 + 0.35 as f32) as usize >= p1_curr_anim as usize {
                //TODO temp location, currently it adds the projectile once at the end, but should add at specific key frames
                if player1.is_attacking && p1_assets.effects.contains_key(&player_current_animation.name) {
                    let mut projectile = (*p1_assets.effects.get(&player_current_animation.name).unwrap()).clone();
                    projectile.position = projectile.position.offset(player1.position.x(), 0);
                    projectile.direction.x = (player2.position.x - player1.position.x).signum();
                    projectile.flipped = player1.dir_related_of_other > 0;
                    projectile.player_owner = player1.id;

                    let target_pos = Point::new(player2.position.x + (projectile.direction.x * 100), projectile.position.y);
                    projectile.target_position = Some(target_pos);
                    projectiles.push(projectile);
                }
            }
*/