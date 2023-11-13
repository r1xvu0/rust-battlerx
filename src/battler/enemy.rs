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
    pub damage: i32,
    pub defense: i32,
    pub speed: f32,
    pub crit_chance: f32,
    pub crit_multi: f32,
}

impl Enemy {
    // pub fn new(enemy_data: Enemy) -> Enemy {
    //     Enemy {
    //         name: enemy_data.name,
    //         health: enemy_data.health,
    //         level: enemy_data.level,
    //         attack: enemy_data.attack,
    //         defense: enemy_data.defense,
    //         speed: enemy_data.speed,
    //         crit_chance: enemy_data.crit_chance,
    //         // name,
    //         // health: 100,
    //         // level: 1,
    //         // attack: 5,
    //         // defense: 4,
    //         // speed: 0.8,
    //         // crit_chance: 0.0,
    //     }
    // }

    pub fn generate(player: &Player) -> Enemy {
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
            damage: enemy.damage,
            defense: enemy.defense,
            speed: enemy.speed,
            crit_chance: enemy.crit_chance,
            crit_multi: enemy.crit_multi,
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        println!(" => {} has {} {}", self.name.red(), self.health.max(0).to_string().bold(), "health left".bold());
    }

    pub fn attack(&self, target: &mut Player) {
        let mut rng = rand::thread_rng();

        let damage_modifier = rng.gen_range(0.9..=1.5);
        let base_damage = ((self.damage as f32 * damage_modifier) as i32).max(0);
        let damage = (base_damage - target.defense).max(0);
        let crit_roll = rng.gen_range(0.0..1.0);

        if crit_roll < self.crit_chance {
            let crit_damage = ((damage as f32) * self.crit_multi) as i32;
            println!("{} {} strikes for {} damage", self.name.red(), "critically".to_string().yellow() , crit_damage.to_string().red().bold());
            target.take_damage(crit_damage);
        } else {
            println!("{} strikes for {} damage", self.name.red(), damage.to_string().red().bold());
            target.take_damage(damage);
        }

    }
}
