extern crate texture;

use std::process;

use texture::master::GameMaster;
use texture::command::GameCommand;
use texture::scenario::Scenario;
use texture::state::State;
use texture::state::BasicState;

// Scenarios
struct Start;
struct Second;

// Commands
struct Exit;

impl Scenario for Start {
    fn load(&self, state: &mut Box<State>) {
        println!("This is the start scenario");
    }

    fn do_action(&self, command: &str, state: &mut Box<State>) {
        println!("Your command was {}", command);
        println!("Loading next scenario...");

        state.set_scenario("second".to_string());
    }
}

impl Scenario for Second {
    fn load(&self, state: &mut Box<State>) {
        println!("This is the second scenario");
    }

    fn do_action(&self, command: &str, state: &mut Box<State>) {
        println!("Your command was {}", command);
        println!("This scenario does nothing");
    }
}

impl GameCommand for Exit {
    fn execute(&self, state: &mut Box<State>) {
        println!("Exiting game");
        process::exit(0);
    }
}

fn main() {
    // Create basic state
    let mut state = BasicState::new();

    // Create game master
    let mut game_master = GameMaster::new(Box::new(state));

    // Add scenarios
    let start = Start;
    let second = Second;

    game_master.add_scenario("start".to_string(), Box::new(start));
    game_master.add_scenario("second".to_string(), Box::new(second));

    // Add commands
    let exitcmd = Exit;

    game_master.add_command("exit".to_string(), Box::new(exitcmd));

    // Start game
    game_master.start_game();
}
