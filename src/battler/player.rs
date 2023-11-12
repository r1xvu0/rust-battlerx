use colored::Colorize;
use rand::Rng;

use super::enemy::Enemy;

pub struct Player {
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub level: i32,
    pub xp: i32,
    pub next_level_xp: i32,
    pub attack: i32,
    pub armor: i32,
    pub speed: f32,
    pub crit_chance: f32,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            health: 100,
            max_health: 100,
            level: 30,
            xp: 0,
            next_level_xp: 100,
            attack: 5,
            armor: 3,
            speed: 1.0,
            crit_chance: 0.25,
        }
    }

    pub fn level_up(&mut self) {
        if self.xp >= self.next_level_xp {
            self.xp = 0;
            self.level += 1;
            self.max_health += 10;
            self.health = self.max_health;
            self.next_level_xp = self.next_level_xp * 2;
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        println!(" => {} has {} health left", self.name.green(), self.health);
    }

    pub fn attack(&self, target: &mut Enemy) {
        let mut rng = rand::thread_rng();
        let crit_roll: f32 = rng.gen_range(0.0..1.0);
        let random_mod: i32 = rng.gen_range(0..10);
        let random_modifier = random_mod * self.level / 10;
        println!("{}", random_modifier);

        if crit_roll < self.crit_chance {
            // let crit_damage = ((self.attack + random_modifier) * 2) - target.armor;
            let crit_damage = ((self.attack + random_modifier) - target.armor) * 2;
            println!("{} {} strikes for {} damage", self.name.green(), "critically".to_string().yellow() , crit_damage);
            target.take_damage(crit_damage);
        } else {
            let damage = (self.attack + random_modifier) - target.armor;
            println!("{} strikes for {} damage", self.name.green(), damage);
            target.take_damage(damage);
        }
    }
}