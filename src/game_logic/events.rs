pub struct Event {
    pub id: i32,
    pub text: String,
    pub portrait_id: String,
    pub options: Vec<String>,
    pub rewards: Option<Rewards>,
    pub details: Option<Details>,
    pub cost: Option<Cost>,
}

pub struct Rewards {
    pub currency: i32,
    pub item_ids: Vec<i32>,
}

pub struct Details {
    pub target: i32,
}

pub struct Cost {
    pub health: i32,
    pub energy: i32,
    pub currency: i32,
    pub items: Vec<i32>,
}
