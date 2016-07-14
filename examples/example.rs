extern crate texture;

use std::process;
use std::collections::HashMap;

use texture::master::GameMaster;
use texture::command::GameCommand;
use texture::scenario::Scenario;


// Custom state object and type
struct MyState {
    flags: HashMap<String, bool>
}

trait CustomState {
    fn new() -> Self;

    /// Initial state
    fn clear(&mut self);

    /// Set a value in the `flags` map to true
    fn set_flag_true(&mut self, name: String);

    /// Set a value in the `flags` map to false
    fn set_flag_false(&mut self, name: String);

    /// Get flag value
    fn get_flag(&self, name: String) -> bool;
}

impl CustomState for MyState {
    fn new() -> MyState {
        MyState {flags: HashMap::new()}
    }

    fn clear(&mut self) {
        self.flags.insert("in_start".to_string(), true);
    }

    fn set_flag_true(&mut self, name: String) {
        self.flags.insert(name, true);
    }

    fn set_flag_false(&mut self, name: String) {
        self.flags.insert(name, false);
    }

    fn get_flag(&self, name: String) -> bool {
        let val = match self.flags.get(&name) {
            Some(s) => { s.clone() },
            None => { false }
        };

        return val;
    }
}


// Scenarios
struct Start;
struct Second;

// Commands
struct Exit;

impl <S: CustomState> Scenario<S> for Start {
    fn load(&self, state: &mut Box<S>) -> Option<String> {
        println!("This is the start scenario");
        println!("Value of in_start: {}", state.get_flag("in_start".to_string()));

        return None;
    }

    fn do_action(&self, command: &str, state: &mut Box<S>) -> Option<String> {
        println!("Your command was {}", command);

        match command {
            "tick" => { println!("ticking"); return Some("_tick".to_string()) }
            _ => {
                println!("Setting in_start to false and loading next scenario...");

                state.set_flag_false("in_start".to_string());
                return Some("second".to_string());
            }
        };
    }
}

impl <S: CustomState> Scenario<S> for Second {
    fn load(&self, state: &mut Box<S>) -> Option<String> {
        println!("This is the second scenario");
        println!("Value of in_start: {}", state.get_flag("in_start".to_string()));

        return None;
    }

    fn do_action(&self, command: &str, state: &mut Box<S>) -> Option<String> {
        println!("Your command was {}", command);
        println!("This scenario does nothing");
        println!("Value of in_start: {}", state.get_flag("in_start".to_string()));

        return None;
    }
}

impl <S: CustomState> GameCommand <S> for Exit {
    fn execute(&self, _: &mut Box<S>) -> Option<String> {
        println!("Exiting game");
        process::exit(0);
    }
}

fn main() {
    // Create custom state
    let mut state = MyState::new();
    state.clear();

    // Create game master
    let mut gm = GameMaster::new(Box::new(state));

    // Add scenarios
    let start = Start;
    let second = Second;

    gm.add_scenario("start".to_string(), Box::new(start));
    gm.add_scenario("second".to_string(), Box::new(second));

    // Add commands
    let exitcmd = Exit;

    gm.add_command("exit".to_string(), Box::new(exitcmd));

    // Start game
    gm.start_game();
}
