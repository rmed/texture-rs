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

pub struct GameMaster<S> {
    // Current scenario
    current: String,
    // Game state
    state: Box<S>,
    // Global game commands
    commands: HashMap<String, Box<GameCommand<S>>>,
    // Scenarios
    scenarios: HashMap<String, Box<Scenario<S>>>
}

impl <S> GameMaster <S> {
    /// Create a new game master using the provided data
    ///
    /// # Examples
    ///
    /// ```
    /// use texture::master::GameMaster;
    ///
    ///
    /// trait MyState {
    ///     fn new() -> Self;
    /// }
    ///
    /// // Create a global state
    /// struct CustomState {
    ///     flag: bool
    /// }
    ///
    /// impl MyState for CustomState {
    ///     fn new() -> CustomState {
    ///         CustomState{ flag: true }
    ///     }
    /// }
    ///
    /// // Create basic state
    /// let mut state = CustomState::new();
    ///
    /// // Create game master
    /// let mut gm = GameMaster::new(Box::new(state));
    /// ```
    pub fn new(state: Box<S>) -> GameMaster<S> {

        GameMaster {
            current: "".to_string(),
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
    /// use texture::command::GameCommand;
    /// use texture::master::GameMaster;
    ///
    ///
    /// trait MyState {
    ///     fn new() -> Self;
    /// }
    ///
    /// // Create a global state
    /// struct CustomState {
    ///     flag: bool
    /// }
    ///
    /// impl MyState for CustomState {
    ///     fn new() -> CustomState {
    ///         CustomState{ flag: true }
    ///     }
    /// }
    ///
    /// struct MyCommand;
    ///
    /// impl <S: MyState> GameCommand <S> for MyCommand {
    ///     // Print message
    ///     fn execute(&self, state: &mut Box<S>) -> Option<String> {
    ///         println!("This is my command");
    ///
    ///         return None;
    ///     }
    /// }
    ///
    /// // Create basic state
    /// let mut state = CustomState::new();
    ///
    /// // Create game master
    /// let mut gm = GameMaster::new(Box::new(state));
    ///
    /// // Create custom command
    /// let command = MyCommand;
    /// gm.add_command("test".to_string(), Box::new(command));
    /// ```
    pub fn add_command(&mut self, name: String, command: Box<GameCommand<S>>) {
        self.commands.insert(name, command);
    }

    /// Insert a new scenario in the map
    ///
    /// # Examples
    ///
    /// ```
    /// use texture::scenario::Scenario;
    /// use texture::master::GameMaster;
    ///
    /// trait MyState {
    ///     fn new() -> Self;
    /// }
    ///
    /// // Create a global state
    /// struct CustomState {
    ///     flag: bool
    /// }
    ///
    /// impl MyState for CustomState {
    ///     fn new() -> CustomState {
    ///         CustomState{ flag: true }
    ///     }
    /// }
    ///
    /// struct ScenarioA;
    ///
    /// impl <S:MyState> Scenario <S> for ScenarioA {
    ///     fn load(&self, state: &mut Box<S>) -> Option<String> {
    ///         println!("This is Scenario A");
    ///
    ///         return None;
    ///     }
    ///
    ///     fn do_action(&self, command: &str, state: &mut Box<S>) -> Option<String> {
    ///         println!("Actions should be parsed here");
    ///
    ///         // Load "scenariob"
    ///         return Some("scenariob".to_string());
    ///     }
    /// }
    ///
    ///
    /// // Create basic state
    /// let mut state = CustomState::new();
    ///
    /// // Create game master
    /// let mut gm = GameMaster::new(Box::new(state));
    ///
    /// // Create scenario
    /// let scenario = ScenarioA;
    /// gm.add_scenario("start".to_string(), Box::new(scenario));
    /// ```
    pub fn add_scenario(&mut self, name: String, scenario: Box<Scenario<S>>) {
        self.scenarios.insert(name, scenario);
    }

    /// Start a new game by calling the main loop
    pub fn start_game(&mut self) {
        self.main_loop();
    }

    /// Load a new scenario, calling its `load()` method
    fn change_scenario(&mut self, name: String) -> Option<String> {
        // Obtain the scenario
        let scenario = match self.scenarios.get(&name) {
            Some(s) => { s },
            _ => {
                panic!("[ERROR] scenario {} not found", name);
            }
        };

        // Load the scenario
        self.current = name;
        return scenario.load(&mut self.state);
    }

    /// Execute a global game command (if any)
    fn exec_game_command(&mut self, command: &str) -> Option<String> {
        let game_command = match self.commands.get(command) {
            Some(f) => { f },
            None => return None
        };

        return game_command.execute(&mut self.state);
    }

    /// Execute the action of the current scenario
    fn exec_current_scenario(&mut self, command: &str) -> Option<String> {
        let scenario = match self.scenarios.get(&self.current) {
            Some(s) => { s },
            _ => {
                panic!("[ERROR] scenario {} not found", self.current);
            }
        };

        let result = scenario.do_action(&command.trim(), &mut self.state);

        return result;
    }

    /// Main game loop
    fn main_loop(&mut self) {
        // Setup linenoise
        linenoise::set_multiline(0);

        // Set first scenario
        self.change_scenario("start".to_string());

        // Infinite game loop
        let mut input = String::new();
        let mut command;

        loop {
            // Get input
            input = match linenoise::input("\n> ") {
                Some(i) => { i },
                None => { continue }
            };

            command = input.clone();

            println!(" ");

            // Try to execute global game commands
            match self.exec_game_command(&command.trim()) {
                Some(r) => { self.change_scenario(r); continue },
                _ => {}
            };

            // No global command found, execute scenario
            match self.exec_current_scenario(&command.trim()) {
                Some(r) => { self.change_scenario(r); },
                _ => {}
            };
        }
    }
}
