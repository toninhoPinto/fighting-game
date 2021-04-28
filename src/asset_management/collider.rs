use parry2d::bounding_volume::AABB;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum ColliderType {
    Hitbox,  //attacking collider
    Hurtbox, //take damage
    Grabbox,
    Grabbablebox,
    Pushbox, //push character
}
#[derive(Debug, Clone)]
pub struct Collider {
    pub collider_type: ColliderType,
    pub name: String,
    pub aabb: AABB,
    pub enabled: bool,
}

