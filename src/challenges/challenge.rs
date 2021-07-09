#[derive(Clone)]
pub struct Challenge {
    pub is_completed: bool,
    pub details: Option<f32>,
}

impl Challenge {
    pub fn new(details: Option<f32>) -> Self {
        Self{
            is_completed: false,
            details
        }
    }
}

pub type ChallengeEvent = fn(&mut Challenge) -> ();
pub type ChallengeAttackEvent = fn(f32, &mut Challenge) -> ();

pub fn highscore_check(value: f32, challenge: &mut Challenge) {
    challenge.is_completed |= challenge.details.unwrap() < value;
}

pub fn instant_fail(challenge: &mut Challenge) {
    challenge.is_completed = false;
}