// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        (self.health == 0).then(|| Self {
            health: 100,
            mana: (self.mana.is_some()).then(|| 100),
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(ref mut mana) => {
                if *mana >= mana_cost {
                    *mana -= mana_cost;
                    2 * mana_cost
                } else {
                    0
                }
            }
            None => {
                if self.health < mana_cost {
                    self.health = 0;
                } else {
                    self.health -= mana_cost;
                }
                0
            }
        }
    }
}
