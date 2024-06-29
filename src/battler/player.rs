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
    pub stat_points: i32,
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
            stat_points: 0,
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

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        println!(" => {} has {} {}", self.name.green(), self.health.max(0).to_string().bold(), "health left".bold());
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
            target.take_damage(crit_damage);
        } else {
            println!("{} strikes for {} damage", self.name.green(), damage.to_string().red().bold());
            target.take_damage(damage);
        }
    }

    pub fn add_exp(&mut self, xp: i32) {
        println!("{} gained {} xp", self.name.green(), xp.to_string().yellow().bold());
        self.xp += xp;
        if self.xp >= self.next_level_xp {
            self.level_up();
        } else {
            println!("{} now has {}/{} xp", self.name.green(), self.xp.to_string().yellow().bold(), self.next_level_xp.to_string().yellow().bold());
        }
    }

    pub fn level_up(&mut self) {
        let old_level = self.level;
        self.level += 1;
        println!("{} has leveled up! {} => {}", self.name.green(), old_level.to_string().yellow().bold(), self.level.to_string().yellow().bold());
        self.max_health += 10;
        self.health = self.max_health;
        self.next_level_xp = self.next_level_xp * 2;
        self.stat_points += 5;
        if self.xp >= self.next_level_xp {
            self.level_up();
        } else {
            println!("{} now has {}/{} xp", self.name.green(), self.xp.to_string().yellow().bold(), self.next_level_xp.to_string().yellow().bold());
        }
    }

    pub fn distribute_stat_points(&mut self) {
        println!("Choose stat to distribute: (health, damage, defense, speed, crit_chance, crit_multi)");
        let mut stat = String::new();
        stdin().read_line(&mut stat).expect("Failed to read line");
        let stat: &str = &stat.trim();

        println!("Enter number of points to distribute into {stat}: ");
        let mut points = String::new();
        stdin().read_line(&mut points).expect("Failed to read line");
        let points: i32 = points.trim().parse().expect("Failed to parse number");

        if self.stat_points < points {
            println!("Not enough stat points!");
        } else {
            match stat {
                "health" => {
                    self.max_health += points * 5;
                    self.health = self.max_health;
                    println!("{} HP Max => {} HP Max", self.health.to_string().yellow().bold(), self.max_health.to_string().yellow().bold());
                    self.stat_points -= points;
                }
                "damage" => {
                    let old_damage = self.damage;
                    self.damage += points;
                    println!("{} Damage => {} Damage", old_damage.to_string().yellow().bold(), self.damage.to_string().yellow().bold());
                    self.stat_points -= points;
                }
                "defense" => {
                    let old_defense = self.defense;
                    self.defense += points;
                    println!("{} Defense => {} Defense", old_defense.to_string().yellow().bold(), self.defense.to_string().yellow().bold());
                    self.stat_points -= points;

                }
                "speed" => {
                    let old_speed = self.speed;
                    self.speed += points as f32 / 100.0;
                    println!("{} Speed => {} Speed", old_speed.to_string().yellow().bold(), self.speed.to_string().yellow().bold());
                    self.stat_points -= points;
                }
                "crit_chance" => {
                    let old_crit_chance = self.crit_chance * 100.0;
                    let rounded_old_crit_chance = (old_crit_chance * 100.0).round() / 100.0;
                    self.crit_chance += (points as f32 / 2.0) / 100.0;
                    let crit_chance_normalized = self.crit_chance * 100.0;
                    let rounded_crit_chance: f32 = (crit_chance_normalized * 100.0).round() / 100.0;
                    println!("{}% Critical Chance => {}% Critical Chance", rounded_old_crit_chance.to_string().yellow().bold(), rounded_crit_chance.to_string().yellow().bold());
                    self.stat_points -= points;
                }
                "crit_multi" => {
                    let old_crit_multi = self.crit_multi;
                    self.crit_multi += points as f32 / 100.0;
                    let crit_multi_normalized = self.crit_multi;
                    println!("{:.5}x Critical Multiplier => {:.5}x Critical Multiplier", old_crit_multi.to_string().yellow().bold(), crit_multi_normalized.to_string().yellow().bold());
                    self.stat_points -= points;
                }
                _ => {
                    println!("Invalid stat");
                }
            }
        }
        self.save().expect("Failed to save player");
    }

    pub fn stat_check(&mut self) {
        println!("{} Stats {}", "=".repeat(25), "=".repeat(25));
        println!("Name: {:>13}", self.name.green().bold());
        println!("Level: {:>9}", self.level.to_string().yellow().bold());
        println!("XP: {:>12}/{}", self.xp.to_string().yellow().bold(), self.next_level_xp.to_string().yellow().bold());
        println!("Health: {:>10}/{} {:>20}", self.health.to_string().red().bold(), self.max_health.to_string().red().bold(), "(+10 per point)".green().bold());
        println!("Damage: {:>8} {:>25}", self.damage.to_string().yellow().bold(), "(+1 per point)".green().bold());
        println!("Defense: {:>7} {:>25}", self.defense.to_string().yellow().bold(), "(+1 per point)".green().bold());
        println!("Speed: {:>9} {:>26}", self.speed.to_string().yellow().bold(), "(+1% per point)".green().bold());
        let crit_chance_normalized: f32 = self.crit_chance * 100.0; 
        let rounded_crit_chance: f32 = (crit_chance_normalized * 100.0).round() / 100.0;
        println!("Crit Chance: {:>4}% {:>26}", rounded_crit_chance.to_string().yellow().bold(), "(+0.5% per point)".green().bold());
        println!("Crit Multi: {:>6.5}x {:>26}", self.crit_multi.to_string().yellow().bold(), "(+0.01x per point)".green().bold());
        println!("{}", "=".repeat(57));

        if self.stat_points > 0 {
            println!("You have {} unspent Stat Points, do you wish to use them Y/N?", self.stat_points.to_string().yellow().bold());
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();
            if input == "Y" || input == "y" {
                self.distribute_stat_points();
            }
        }
    }

    pub fn change_location(&mut self) {
        todo!();
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
            stat_points: self.stat_points,
            location: self.location.clone(),
        };
        player
    }
}