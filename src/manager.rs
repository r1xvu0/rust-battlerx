 use std::io::stdin;

use colored::Colorize;

use crate::battler::{self, enemy::Enemy, player};

pub struct Manager {
    manager_state: ManagerState,
    player: player::Player,
}

#[derive(PartialEq)]
pub enum ManagerState {
    Battling,
    City,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            manager_state: ManagerState::City,
            player: player::Player::load().expect("Failed to load player"),
        }
    }

    pub fn start(&mut self) {
        match self.manager_state {
            ManagerState::Battling => {
                self.battle();
            }
            ManagerState::City => {
                self.menu();
            }
        }
    }

    fn menu(&mut self) {
        loop {
            println!("{} {}", "=".repeat(25), "=".repeat(25));
            println!{"{} ({}) | HP: {} / {} | EXP: {} / {}", self.player.name.green().bold(), self.player.level.to_string().yellow().bold(), self.player.health.to_string().red().bold(), self.player.max_health.to_string().red().bold(), self.player.xp.to_string().yellow().bold(), self.player.next_level_xp.to_string().yellow().bold()};
            println!("{} {}", "=".repeat(25), "=".repeat(25));
            println!("Welcome to the city!");
            println!("{} {}", "=".repeat(25), "=".repeat(25));
            println!("b) Battle | c) Change Location | h) Heal | s) Stat Check | m) Manual | q) Quit");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim() {
                "b" => {
                    self.change_state(ManagerState::Battling);
                    self.start();
                }
                "c" => {
                    self.player.change_location();
                    self.start();
                }
                "m" => {
                    todo!("Implement Manual");
                }
                "h" => {
                    self.heal();
                }
                "s" => {
                    // self.player.distribute_stat_points();
                    self.player.stat_check();
                }
                "q" => {
                    std::process::exit(0);
                }
                _ => {
                    println!("Invalid input");
                }
            }
        }
    }

    fn battle(&mut self) {
        let player = self.player.clone();
        let enemy: Enemy = Enemy::generate(&player);

        let mut battler = battler::Battler::new(player, enemy);
        battler.fight(self);
        if battler.player.health <= 0 {
            battler.player.health = 1;
        }
        
        self.player = battler.player;
        self.player.save().expect("Failed to save player");
        self.start();
    }

    pub fn change_state(&mut self, state: ManagerState) {
        self.manager_state = state;
    }

    fn heal(&mut self) {
        self.player.health = self.player.max_health;
        println!("Player healed!");
    }
}
