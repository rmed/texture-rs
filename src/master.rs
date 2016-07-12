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

use linenoise;

use command::GameCommand;
use scenario::Scenario;
use state::State;

pub struct GameMaster {
    // Game state
    state: Box<State>,
    // Global game commands
    commands: HashMap<String, Box<GameCommand>>,
    // Scenarios
    scenarios: HashMap<String, Box<Scenario>>
}

impl GameMaster {
    /// Create a new game master using the provided data
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use texture::master::GameMaster;
    /// use texture::state::State;
    /// use texture::state::BasicState;
    ///
    /// // Create basic state
    /// let mut state = BasicState::new();
    ///
    /// // Create game master
    /// let mut game_master = GameMaster::new(Box::new(state));
    /// ```
    pub fn new(state: Box<State>) -> GameMaster {

        GameMaster {
            state: state,
            commands: HashMap::new(),
            scenarios: HashMap::new()
        }
    }

    /// Insert a new global command in the map
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use texture::command::GameCommand;
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
    /// // Create basic state
    /// let mut state = BasicState::new();
    ///
    /// // Create game master
    /// let mut game_master = GameMaster::new(Box::new(state));
    ///
    /// // Create custom command
    /// let command = MyCommand;
    /// game_master.add_command("test".to_string(), Box::new(command));
    /// ```
    pub fn add_command(&mut self, name: String, command: Box<GameCommand>) {
        self.commands.insert(name, command);
    }

    /// Insert a new scenario in the map
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use texture::scenario::Scenario;
    /// use texture::master::GameMaster;
    /// use texture::state::State;
    /// use texture::state::BasicState;
    ///
    /// struct ScenarioA;
    ///
    /// impl Scenario for ScenarioA {
    ///     fn load(&self, state: &mut Box<State>) {
    ///         println!("This is Scenario A");
    ///     }
    ///
    ///     fn do_action(&self, command: &str, state: &mut Box<State>) {
    ///         println!("Actions should be parsed here");
    ///         // Load "scenariob"
    ///         state.set_scenario("scenariob".to_string());
    ///     }
    /// }
    ///
    /// // Create basic state
    /// let mut state = BasicState::new();
    ///
    /// // Create game master
    /// let mut game_master = GameMaster::new(Box::new(state));
    ///
    /// // Create scenario
    /// let scenario = ScenarioA;
    /// game_master.add_scenario("start".to_string(), Box::new(scenario));
    /// ```
    pub fn add_scenario(&mut self, name: String, scenario: Box<Scenario>) {
        self.scenarios.insert(name, scenario);
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
            Some(f) => { f },
            None => return false
        };

        game_command.execute(&mut self.state);
        return true;
    }

    /// Main game loop
    fn main_loop(&mut self) {
        // Setup linenoise
        linenoise::set_multiline(0);

        // Set first scenario
        self.state.set_scenario("start".to_string());
        self.change_scenario();

        // Infinite game loop
        let mut input = String::new();
        let mut command;
        let mut current_scenario;

        loop {
            // Get input
            input = match linenoise::input("\n> ") {
                Some(i) => { i },
                None => { continue }
            };

            command = input.clone();

            println!(" ");

            // Try to execute global game commands
            if !self.exec_game_command(&command.trim()) {

                // Perform scenario action
                current_scenario = self.state.get_current_scenario();

                match self.scenarios.get(&current_scenario) {
                    Some(s) => s.do_action(&command.trim(), &mut self.state),
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
