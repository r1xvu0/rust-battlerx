pub mod enemy;
pub mod player;

use chrono::{Local, Utc};
use enemy::Enemy;
use player::Player;
use std::{time::{Duration, Instant}, thread::sleep};
use colored::*;

use crate::manager::{Manager, self};

pub struct Battler {
    pub player: Player,
    pub enemy: Enemy,
}

impl Battler {
    pub fn new(player: Player, enemy: Enemy) -> Battler {
        Battler { player, enemy }
    }

    pub fn fight(&mut self, manager: &mut Manager) {
        println!("{} is fighting {}", self.player.name, self.enemy.name);
        sleep(Duration::from_secs(2));

        let player_attack_interval = Duration::from_secs_f32(2.5 / self.player.speed);
        let enemy_attack_interval = Duration::from_secs_f32(2.5 / self.enemy.speed);

        let mut next_player_attack = Instant::now() + player_attack_interval;
        let mut next_enemy_attack = Instant::now() + enemy_attack_interval;

        while self.player.health > 0 && self.enemy.health > 0 {
            // convert time to regular 24h format
            let current_time = Instant::now();

            let time = Local::now();
            let timed = time.format("%H:%M:%S").to_string();

            if next_player_attack <= current_time {
                println!("{} {} {}", "=".repeat(16), timed,  "=".repeat(16));
                // println!("{}", timed);
                self.player.attack(&mut self.enemy);
                next_player_attack = current_time + player_attack_interval;
                println!("{}", "=".repeat(42));
            }

            if next_enemy_attack <= current_time {
                println!("{} {} {}", "=".repeat(16), timed,  "=".repeat(16));
                self.enemy.attack(&mut self.player);
                next_enemy_attack = current_time + enemy_attack_interval;
                println!("{}", "=".repeat(42));
            }

            if self.player.health <= 0 {
                println!("\n\n");
                println!("{:>10} {:<10} {}", self.player.name.green().bold(), "", "has died".bold());
                println!("\n\n");
                manager.change_state(manager::ManagerState::City);
            } else if self.enemy.health <= 0 {
                println!("\n\n");
                println!("{:>10} {:>10} {}", self.enemy.name.red().bold(), "", "has perished".bold());
                println!("\n");
                println!("{:>10} {:>10} {}", self.player.name.green().bold(), "", "has won".bold());
                println!("\n\n");
                manager.change_state(manager::ManagerState::City);
            }
        }
        
    }
}
