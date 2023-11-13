use std::{fs::File, io::Read};

use colored::Colorize;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::player::Player;

#[derive(Serialize, Deserialize)]
pub struct Enemy {
    pub name: String,
    pub health: i32,
    pub level: i32,
    pub attack: i32,
    pub armor: i32,
    pub speed: f32,
    pub crit_chance: f32,
}

impl Enemy {
    // pub fn new(enemy_data: Enemy) -> Enemy {
    //     Enemy {
    //         name: enemy_data.name,
    //         health: enemy_data.health,
    //         level: enemy_data.level,
    //         attack: enemy_data.attack,
    //         armor: enemy_data.armor,
    //         speed: enemy_data.speed,
    //         crit_chance: enemy_data.crit_chance,
    //         // name,
    //         // health: 100,
    //         // level: 1,
    //         // attack: 5,
    //         // armor: 4,
    //         // speed: 0.8,
    //         // crit_chance: 0.0,
    //     }
    // }

    pub fn generate(player: &Player) -> Enemy {
        // todo!("Generate enemy");
        let location = &player.location;
        let file_path = format!(
            "data/enemies/{}.json",
            location.replace(" ", "_").to_lowercase()
        );

        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        let enemies: Vec<Enemy> = serde_json::from_str(&contents).expect("Failed to parse JSON");
        let random_index = rand::thread_rng().gen_range(0..enemies.len());
        let enemy = &enemies[random_index];

        println!("{} has appeared!", enemy.name.red().bold());

        Enemy {
            name: enemy.name.clone(),
            health: enemy.health,
            level: enemy.level,
            attack: enemy.attack,
            armor: enemy.armor,
            speed: enemy.speed,
            crit_chance: enemy.crit_chance,
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        println!(" => {} has {} health left", self.name.red(), self.health);
    }

    pub fn attack(&self, target: &mut Player) {
        let mut damage = self.attack - target.armor;
        if damage < 0 {
            damage = 0;
        }
        println!("{} strikes for {} damage", self.name.red(), damage);
        target.take_damage(damage);
    }
}
