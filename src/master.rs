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
use std::cell::RefCell;
use std::rc::Rc;

use linenoise;

use command::GameCommand;
use scenario::{Loader, Scenario};
use util::TICK;
use util::LOAD;

pub struct GameMaster<S> {
    // Current scenario
    current: Rc<RefCell<Scenario<S>>>,
    // Scenario loader
    loader: Rc<RefCell<Loader<S>>>,
    // Game state
    state: Rc<RefCell<S>>,
    // Global game commands
    commands: HashMap<String, Box<GameCommand<S>>>,
}

impl <S> GameMaster <S> {
    /// Create a new game master using the provided data
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    ///
    /// use texture::master::GameMaster;
    /// use texture::scenario::{Loader, Scenario};
    /// use texture::util::TICK;
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
    /// // Create dummy scenario
    /// struct MyScenario;
    ///
    /// impl <S: MyState> Scenario <S> for MyScenario {
    ///
    ///     fn load(&self, state: &Rc<RefCell<S>>,
    ///             loader: &Rc<RefCell<Loader<S>>>)
    ///             -> i32 {
    ///         println!("Test");
    ///
    ///         TICK
    ///     }
    ///
    ///     fn do_action(&self, command: &str, state: &Rc<RefCell<S>>,
    ///                  loader: &Rc<RefCell<Loader<S>>>)
    ///                  -> i32 {
    ///         println!("Action: {}", command);
    ///
    ///         TICK
    ///     }
    /// }
    ///
    /// // Create basic state
    /// let state = Rc::new(RefCell::new(CustomState::new()));
    ///
    /// // Create start scenario
    /// let start = Rc::new(RefCell::new(MyScenario));
    ///
    /// // Create game master
    /// let mut gm = GameMaster::new(state, start);
    /// ```
    pub fn new(state: Rc<RefCell<S>>, start: Rc<RefCell<Scenario<S>>>)
               -> GameMaster<S> {
        let mut loader = Loader::new();
        loader.set_scenario(start.clone());

        GameMaster {
            current: start,
            loader: Rc::new(RefCell::new(loader)),
            state: state,
            commands: HashMap::new(),
        }
    }

    /// Insert a new global command in the map
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    ///
    /// use texture::command::GameCommand;
    /// use texture::master::GameMaster;
    /// use texture::scenario::{Loader, Scenario};
    /// use texture::util::TICK;
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
    /// // Create dummy scenario
    /// struct MyScenario;
    ///
    /// impl <S: MyState> Scenario <S> for MyScenario {
    ///
    ///     fn load(&self, state: &Rc<RefCell<S>>,
    ///             loader: &Rc<RefCell<Loader<S>>>)
    ///             -> i32 {
    ///         println!("Test");
    ///
    ///         TICK
    ///     }
    ///
    ///     fn do_action(&self, command: &str, state: &Rc<RefCell<S>>,
    ///                  loader: &Rc<RefCell<Loader<S>>>)
    ///                  -> i32 {
    ///         println!("Action: {}", command);
    ///
    ///         TICK
    ///     }
    /// }
    ///
    /// // Create command
    /// struct MyCommand;
    ///
    /// impl <S: MyState> GameCommand <S> for MyCommand {
    ///     // Print message
    ///     fn execute(&self, state: &Rc<RefCell<S>>,
    ///                loader: &Rc<RefCell<Loader<S>>>)
    ///                -> i32 {
    ///         println!("This is my command");
    ///
    ///         TICK
    ///     }
    /// }
    ///
    /// // Create basic state
    /// let state = Rc::new(RefCell::new(CustomState::new()));
    ///
    /// // Create start scenario
    /// let start = Rc::new(RefCell::new(MyScenario));
    ///
    /// // Create game master
    /// let mut gm = GameMaster::new(state, start);
    ///
    /// // Create custom command
    /// let command = MyCommand;
    /// gm.add_command("test".to_string(), Box::new(command));
    /// ```
    pub fn add_command(&mut self, name: String, command: Box<GameCommand<S>>) {
        self.commands.insert(name, command);
    }

    /// Start a new game by calling the main loop
    pub fn start_game(&mut self) {
        self.main_loop();
    }

    /// Load scenario from the loader and call `load()` method
    fn load_scenario(&mut self) -> i32 {
        self.current = self.loader.borrow().get_scenario();

        println!(" ");

        self.current.borrow().load(&self.state, &self.loader)
    }

    /// Execute a global game command (if any)
    fn exec_game_command(&mut self, command: &str) -> i32 {
        let game_command = match self.commands.get(command) {
            Some(f) => { f },
            None => return TICK
        };

        game_command.execute(&self.state, &self.loader)
    }

    /// Execute the action of the current scenario
    fn exec_current_scenario(&mut self, command: &str) -> i32 {
        let result = self.current.borrow().do_action(
            &command.trim(),
            &self.state,
            &self.loader
        );

        result
    }

    /// Main game loop
    fn main_loop(&mut self) {
        // Setup linenoise
        linenoise::set_multiline(0);

        // Infinite game loop
        let mut input = String::new();
        let mut command;

        // Load starting scenario
        self.current.borrow().load(&self.state, &self.loader);

        loop {
            // Get input
            input = match linenoise::input("\n> ") {
                Some(i) => { i },
                None => { continue }
            };

            command = input.clone();

            println!(" ");

            // Try to execute global game commands
            self.exec_game_command(&command.trim());
            match self.exec_game_command(&command.trim()) {
                LOAD => { self.load_scenario(); continue },
                _ => {}
            };

            // No global command found, execute scenario
            match self.exec_current_scenario(&command.trim()) {
                LOAD => { self.load_scenario(); },
                _ => {}
            };
        }
    }
}
