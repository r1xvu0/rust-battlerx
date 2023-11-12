use colored::Colorize;

use super::player::Player;

pub struct Enemy {
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub armor: i32,
    pub speed: f32,
    pub crit_chance: f32,

}

impl Enemy {
    pub fn new(name: String) -> Enemy {
        Enemy {
            name,
            health: 100,
            max_health: 100,
            attack: 5,
            armor: 4,
            speed: 0.8,
            crit_chance: 0.0,
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        println!(" => {} has {} health left", self.name.red(), self.health);
    }

    pub fn attack(&self, target: &mut Player) {
        let damage = self.attack - target.armor;
        println!("{} strikes for {} damage", self.name.red(), damage);        
        target.take_damage(damage);
    }
}