
// use battler::{player, enemy};
use manager::ManagerState;

mod battler;
mod manager;

fn main() {
    let mut manager = manager::Manager::new();

    manager.change_state(ManagerState::City);

    manager.start();
}