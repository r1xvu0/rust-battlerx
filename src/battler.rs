pub mod enemy;
pub mod player;

use enemy::Enemy;
use player::Player;
use std::time::{Duration, Instant};
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

    // pub fn status(&self) {
    //     println!("{}", "=".repeat(25));
    //     println!("{:<10} {:<5} {}", self.player.name, "|".to_string(), self.enemy.name);
    //     println!("{:<10} {:<5} {}", self.player.health, "|".to_string(), self.enemy.health);
    //     println!("{}", "=".repeat(25));
    // }

    pub fn fight(&mut self, manager: &mut Manager) {
        println!("{} is fighting {}", self.player.name, self.enemy.name);

        let player_attack_interval = Duration::from_secs_f32(2.5 / self.player.speed);
        let enemy_attack_interval = Duration::from_secs_f32(2.5 / self.enemy.speed);

        let mut next_player_attack = Instant::now();
        let mut next_enemy_attack = Instant::now();

        while self.player.health > 0 && self.enemy.health > 0 {
            let current_time = Instant::now();

            if next_player_attack <= current_time {
                println!("{}", "=".repeat(32));
                self.player.attack(&mut self.enemy);
                next_player_attack = current_time + player_attack_interval;
                println!("{}", "=".repeat(32));
            }

            if next_enemy_attack <= current_time {
                println!("{}", "=".repeat(32));
                self.enemy.attack(&mut self.player);
                next_enemy_attack = current_time + enemy_attack_interval;
                println!("{}", "=".repeat(32));
            }

            if self.player.health <= 0 {
                println!("\n\n");
                println!("{:>10} {:<10} {}", self.player.name.green().bold(), "", "has died".bold());
                println!("\n\n");
                manager.change_state(manager::ManagerState::City);
            } else if self.enemy.health <= 0 {
                println!("\n\n");
                println!("{:>10} {:>10} {}", self.enemy.name.red().bold(), "", "has perished".bold());
                // println!("{} has {} health left", self.player.name.green(), self.player.health);
                println!("\n\n");
                println!("{:>10} {:>10} {}", self.player.name.green().bold(), "", "has won".bold());
                println!("\n\n");
                manager.change_state(manager::ManagerState::City);
                // manager.start()
            }
        }
        
    }
}
