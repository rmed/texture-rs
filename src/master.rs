// MIT License
//
// Copyright (c) 2016 Rafael Medina García <rafamedgar@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::collections::HashMap;
use std::io;
use command;
use scenario;
use state;

pub struct GameMaster {
    // Game state
    state: Box<state::State>,
    // Global game commands
    commands: HashMap<String, Box<command::GameCommand>>,
    // Scenarios
    scenarios: HashMap<String, Box<scenario::Scenario>>
}

impl GameMaster {
    /// Create a new game master using the provided data
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use texture::command::GameCommand;
    /// use texture::scenario::Scenario;
    /// use texture::master::GameMaster;
    /// use texture::state::State;
    /// use texture::state::BasicState;
    ///
    /// struct MyCommand;
    ///
    /// impl GameCommand for MyCommand {
    ///     // Print message
    ///     fn execute(&self, state: &mut Box<State>) {
    ///         println!("This is my command");
    ///     }
    /// }
    ///
    /// struct ScenarioA;
    /// struct ScenarioB;
    ///
    /// impl Scenario for ScenarioA {
    ///     fn load(&self, state: &mut Box<State>) {
    ///         println!("This is Scenario A");
    ///     }
    ///
    ///     fn do_action(&self, command: String, state: &mut Box<State>) {
    ///         println!("Actions should be parsed here");
    ///         // Load "scenariob"
    ///         state.set_scenario("scenariob".to_string());
    ///     }
    /// }
    ///
    /// impl Scenario for ScenarioB {
    ///     fn load(&self, state: &mut Box<State>) {
    ///         println!("This is Scenario B");
    ///     }
    ///
    ///     fn do_action(&self, command: String, state: &mut Box<State>) {
    ///         println!("Actions should be parsed here");
    ///         // Load "start"
    ///         state.set_scenario("start".to_string());
    ///     }
    /// }
    ///
    /// // Create basic state
    /// let mut state: Box<State> = Box::new(BasicState::new());
    ///
    /// // Create custom commands
    /// let command: Box<GameCommand> = Box::new(MyCommand);
    ///
    /// let mut commands: HashMap<String, Box<GameCommand>> = HashMap::new();
    /// commands.insert("test".to_string(), command);
    ///
    /// // Create scenarios
    /// let scenarioa: Box<Scenario> = Box::new(ScenarioA);
    /// let scenariob: Box<Scenario> = Box::new(ScenarioB);
    ///
    /// let mut scenarios: HashMap<String, Box<Scenario>> = HashMap::new();
    /// scenarios.insert("start".to_string(), scenarioa);
    /// scenarios.insert("scenariob".to_string(), scenariob);
    ///
    ///
    /// let mut game_master = GameMaster::new(state, commands, scenarios);
    pub fn new(
        state: Box<state::State>,
        commands: HashMap<String, Box<command::GameCommand>>,
        scenarios: HashMap<String, Box<scenario::Scenario>>) -> GameMaster {

        GameMaster {
            state: state,
            commands: commands,
            scenarios: scenarios
        }
    }

    /// Start a new game by calling the main loop
    pub fn start_game(&mut self) {
        self.main_loop();
    }

    /// Load a new scenario, calling its `load()` method
    fn change_scenario(&mut self) {
        // Obtain the scenario
        let scenario_name = self.state.get_next_scenario();

        let scenario = match self.scenarios.get(&scenario_name) {
            Some(s) => { s },
            _ => {
                println!("[ERROR] scenario {} not found", scenario_name);
                return
            }
        };

        // Load the scenario
        scenario.load(&mut self.state);
        self.state.load_scenario();
    }

    /// Execute a global game command (if any)
    ///
    /// Returns a boolean indicating whether or not a game command has been
    /// executed
    fn exec_game_command(&mut self, command: &str) -> bool {
        let game_command = match self.commands.get(command) {
            Some(f) => { f }
            _ => return false
        };

        game_command.execute(&mut self.state);
        return true;
    }

    /// Main game loop
    fn main_loop(&mut self) {
        // Set first scenario
        self.state.set_scenario("start".to_string());
        self.change_scenario();

        // Infinite game loop
        let mut input = String::new();
        let mut command;
        let mut current_scenario;

        loop {
            // Get input
            print!("\n> ");

            input.clear();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {},
                Err(e) => {
                    println!("error: {}", e);
                    continue
                }
            };

            command = input.clone();

            println!(" ");

            // Try to execute global game commands
            if !self.exec_game_command(&command) {

                // Perform scenario action
                current_scenario = self.state.get_current_scenario();

                match self.scenarios.get(&current_scenario) {
                    Some(s) => s.do_action(command, &mut self.state),
                    _ => {
                        println!("[ERROR] scenario {} not found", current_scenario);
                        return
                    }
                };
            }

            // Check if a scenario must be loaded
            if self.state.has_next_scenario() {
                self.change_scenario();
            }
        }
    }
}