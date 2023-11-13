
// use battler::{player, enemy};
use manager::ManagerState;

mod battler;
mod manager;

fn main() {
    // let player = player::Player::new("Cloud".to_string());
    // let enemy = enemy::Enemy::new("Behemot".to_string());

    // let mut battler = battler::Battler::new(player, enemy);
    // battler.fight();

    let mut manager = manager::Manager::new();
    manager.change_state(ManagerState::City);
    manager.start();

    // manager.change_state(ManagerState::City);
    // manager.start();

}