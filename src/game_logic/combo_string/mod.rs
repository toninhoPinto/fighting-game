
pub struct Combo {
    pub counter: u32,
    pub time_limit: f64,
    pub time_since_last: f64,
}

impl Combo {
    pub fn new(time_limit: f64) -> Self {
        Self {
            counter: 0,
            time_limit,
            time_since_last: 0f64,
        }
    }

    pub fn increment_combo(&mut self) {
        self.counter += 1;
        self.time_since_last = 0f64;
    }
    
    pub fn manage_combo(&mut self, time_passed: f64) {
        if self.counter > 0 {

            self.time_since_last += time_passed;

            if self.time_since_last >= self.time_limit {
                self.counter = 0;
                self.time_since_last = 0f64;
            }

        }   
    }

    pub fn render(&self) -> Option<u32> {
        return if self.counter > 2 {
            Some(self.counter)
        } else {
            None 
        }
    }
}


