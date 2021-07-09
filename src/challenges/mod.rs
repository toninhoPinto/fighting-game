use self::challenge::{Challenge, ChallengeAttackEvent, ChallengeEvent, highscore_check};

pub mod challenge;





#[derive(Clone)]
pub struct ChallengeManager {
    pub on_update: Vec<(ChallengeEvent, Challenge)>,

    pub on_heal: Vec<(ChallengeEvent, Challenge)>,
    pub on_hurt: Vec<(ChallengeEvent, Challenge)>,

    pub on_attack: Vec<(ChallengeEvent, Challenge)>,
    pub on_hit: Vec<(ChallengeAttackEvent, Challenge)>,
    pub on_kill: Vec<(ChallengeEvent, Challenge)>,

    pub on_jump: Vec<(ChallengeEvent, Challenge)>,
}

impl ChallengeManager {
    
    pub fn new() -> Self {
        Self{
            on_update: Vec::new(),

            on_heal: Vec::new(),
            on_hurt: Vec::new(),
        
            on_attack: Vec::new(),
            on_hit: Vec::new(),
            on_kill: Vec::new(),
        
            on_jump: Vec::new(),
        
        }
    }

    pub fn register(&mut self, challege_id: i32, challenge: Challenge) {
        match challege_id {
            1 => {self.on_hit.push((highscore_check as ChallengeAttackEvent, challenge))}
            _ => {}
        }
    }

    pub fn get_result(&self) -> bool {
        let all = self.on_update.iter().map(|c| c.1.is_completed).chain(
            self.on_heal.iter().map(|c| c.1.is_completed)
        ).chain(
            self.on_hurt.iter().map(|c| c.1.is_completed)
        ).chain(
            self.on_attack.iter().map(|c| c.1.is_completed)
        ).chain(
            self.on_hit.iter().map(|c| c.1.is_completed)
        ).chain(
            self.on_kill.iter().map(|c| c.1.is_completed)
        ).chain(
            self.on_jump.iter().map(|c| c.1.is_completed)
        ).reduce(|a, b| {
            a || b
        });

        if let Some(all) = all {
            all
        } else {
            false
        }
    }

}


