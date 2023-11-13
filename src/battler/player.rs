use colored::Colorize;
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::{io::stdin, error::Error, fs};

use super::enemy::Enemy;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub level: i32,
    pub xp: i32,
    pub next_level_xp: i32,
    pub damage: i32,
    pub defense: i32,
    pub speed: f32,
    pub crit_chance: f32,
    pub crit_multi: f32,
    pub location: String,
}

impl Player {
    pub fn new() -> Player {
        println!("Enter player name: ");
        let mut name = String::new();
        stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim().to_string();
        Player {
            name,
            health: 100,
            max_health: 100,
            level: 1,
            xp: 0,
            next_level_xp: 100,
            damage: 5,
            defense: 3,
            speed: 1.0,
            crit_chance: 0.25,
            crit_multi: 1.5,
            location: "Silver City".to_string(),
        }
    }

    pub fn load() -> Result<Player, Box<dyn Error>> {
        let file_path = "data/player_data.json";
        if fs::metadata(file_path).is_ok() {
            let player_data = fs::read_to_string(file_path)?;
            let player: Player = serde_json::from_str(&player_data)?;
            Ok(player)
        } else {
            let new_player = Player::new();
            new_player.save()?;
            println!("New player created and possibly saved?");
            Ok(new_player)
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let file_path = "data/player_data.json";
        let player_data = serde_json::to_string(self)?;
        fs::write(file_path, player_data)?;
        Ok(())
    }

    // pub fn level_up(&mut self) {
    //     if self.xp >= self.next_level_xp {
    //         self.xp = 0;
    //         self.level += 1;
    //         self.max_health += 10;
    //         self.health = self.max_health;
    //         self.next_level_xp = self.next_level_xp * 2;
    //     }
    // }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        println!(" => {} has {} {}", self.name.green(), self.health.to_string().bold(), "health left".bold());
    }

    pub fn attack(&self, target: &mut Enemy) {
        let mut rng = rand::thread_rng();
        
        let damage_modifier = rng.gen_range(0.9..=1.5);
        let base_damage = ((self.damage as f32 * damage_modifier) as i32).max(0);
        let damage = (base_damage - target.defense).max(0);

        let crit_roll = rng.gen_range(0.0..1.0);

        if crit_roll < self.crit_chance {
            let crit_damage = ((damage as f32) * self.crit_multi) as i32;
            println!("{} {} strikes for {} damage", self.name.green(), "critically".to_string().yellow() , crit_damage.to_string().red().bold());
            target.take_damage(damage);
        } else {
            println!("{} strikes for {} damage", self.name.green(), damage.to_string().red().bold());
            target.take_damage(damage);
        }
    }

    pub fn clone(&mut self) -> Player {
        let player = Player {
            name: self.name.clone(),
            health: self.health,
            max_health: self.max_health,
            level: self.level,
            xp: self.xp,
            next_level_xp: self.next_level_xp,
            damage: self.damage,
            defense: self.defense,
            speed: self.speed,
            crit_chance: self.crit_chance,
            crit_multi: self.crit_multi,
            location: self.location.clone(),
        };
        player
    }
}