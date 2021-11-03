// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {

    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let new_mana = if self.level >= 10 {
                Some(100)
            } else {
                None
            };
            Some(Self { health: 100, mana: new_mana, level: self.level })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(value) => {
                if mana_cost > value {
                    0
                } else {
                    self.mana = Some(value - mana_cost);
                    mana_cost * 2
                }
            },
            None => {
                if self.health > mana_cost {
                    self.health -= mana_cost;
                } else {
                    self.health = 0;
                }
                0
            }
        }
    }
}
