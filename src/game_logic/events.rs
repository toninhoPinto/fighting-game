pub struct Event {
    pub id: i32,
    pub event_type: EventType,
    pub text: String,
    pub portrait_id: String,
    pub rewards: Option<Rewards>,
    pub details: Option<Challenge>,
    pub cost: Option<Cost>,
}

#[derive(PartialEq, Debug)]
pub enum EventType {
    Challenge,
    TradeOffer,
    LevelMod,
    WorldMod,
}

pub struct Rewards {
    pub currency: i32,
    pub item_ids: Vec<i32>,
}

pub struct Challenge {
    pub target: i32,
}

pub struct Cost {
    pub health: i32,
    pub currency: i32,
    pub items: Vec<i32>,
}
