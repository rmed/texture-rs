extern crate texture;

use std::cell::RefCell;
use std::rc::Rc;
use std::process;
use std::collections::HashMap;

use texture::master::GameMaster;
use texture::command::GameCommand;
use texture::scenario::{Loader, Scenario};
use texture::util::{TICK, LOAD};


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
    fn load(&self, state: &Rc<RefCell<S>>, loader: &Rc<RefCell<Loader<S>>>) -> i32 {
        println!("This is the start scenario");
        println!("Value of in_start: {}", state.borrow().get_flag("in_start".to_string()));

        TICK
    }

    fn do_action(&self, command: &str, state: &Rc<RefCell<S>>, loader: &Rc<RefCell<Loader<S>>>) -> i32 {
        println!("Your command was {}", command);

        match command {
            "tick" => { println!("ticking"); TICK }
            _ => {
                println!("Setting in_start to false and loading next scenario...");

                state.borrow_mut().set_flag_false("in_start".to_string());
                loader.borrow_mut().set_scenario(Rc::new(RefCell::new(Second)));

                LOAD
            }
        }
    }
}

impl <S: CustomState> Scenario<S> for Second {
    fn load(&self, state: &Rc<RefCell<S>>, loader: &Rc<RefCell<Loader<S>>>) -> i32 {
        println!("This is the second scenario");
        println!("Value of in_start: {}", state.borrow().get_flag("in_start".to_string()));

        TICK
    }

    fn do_action(&self, command: &str, state: &Rc<RefCell<S>>, loader: &Rc<RefCell<Loader<S>>>) -> i32 {
        println!("Your command was {}", command);
        println!("This scenario does nothing");
        println!("Value of in_start: {}", state.borrow().get_flag("in_start".to_string()));

        TICK
    }
}

impl <S: CustomState> GameCommand <S> for Exit {
    fn execute(&self, state: &Rc<RefCell<S>>, loader: &Rc<RefCell<Loader<S>>>) -> i32 {
        println!("Exiting game");
        process::exit(0);
    }
}

fn main() {
    // Create custom state
    let mut state = MyState::new();
    state.clear();
    let state_cell = Rc::new(RefCell::new(state));

    // Add scenario
    let start = Rc::new(RefCell::new(Start));

    // Create game master
    let mut gm = GameMaster::new(state_cell, start);

    // Add commands
    let exitcmd = Exit;

    gm.add_command("exit".to_string(), Box::new(exitcmd));

    // Start game
    gm.start_game();
}
