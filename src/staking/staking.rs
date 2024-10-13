use std::collections::HashMap;

pub struct Staking {
    pub stakes: HashMap<String, u64>, // Mapowanie adresów do staków
}

impl Staking {
    pub fn new() -> Staking {
        Staking {
            stakes: HashMap::new(),
        }
    }

    pub fn add_stake(&mut self, validator: String, amount: u64) {
        let entry = self.stakes.entry(validator).or_insert(0);
        *entry += amount;
    }

    pub fn remove_stake(&mut self, validator: String, amount: u64) -> bool {
        if let Some(stake) = self.stakes.get_mut(&validator) {
            if *stake >= amount {
                *stake -= amount;
                if *stake == 0 {
                    self.stakes.remove(&validator);
                }
                return true;
            }
        }
        false
    }

    pub fn total_stake(&self) -> u64 {
        self.stakes.values().sum()
    }

    pub fn select_validator(&self) -> Option<String> {
        let total = self.total_stake();
        if total == 0 {
            return None;
        }

        let mut rng = rand::thread_rng();
        let random_value: u64 = rand::Rng::gen_range(&mut rng, 1..=total);

        let mut cumulative = 0;
        for (validator, stake) in &self.stakes {
            cumulative += stake;
            if random_value <= cumulative {
                return Some(validator.clone());
            }
        }

        None
    }
}
