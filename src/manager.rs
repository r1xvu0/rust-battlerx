use std::{io::stdin, thread::sleep, time::Duration};

use crate::battler::{self, enemy, player};

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
            player: player::Player::new("Cloud".to_string()),
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
            println!("Welcome to the city!");
            println!("{} {}", "=".repeat(25), "=".repeat(25));
            println!("b) Battle | m) Manual | q) Quit");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim() {
                "b" => {
                    self.change_state(ManagerState::Battling);
                    self.start();
                }
                _ => {
                    println!("Invalid input");
                }
            }
        }
    }

    fn battle(&mut self) {
        let player = player::Player::new("Cloud".to_string());
        let enemy = enemy::Enemy::new("Behemot".to_string());

        let mut battler = battler::Battler::new(player, enemy);
        battler.fight(self);
        println!("Player has {} health left", battler.player.health);
        self.start();
    }

    pub fn change_state(&mut self, state: ManagerState) {
        self.manager_state = state;
    }
}
