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

/// Basic state structure
///
/// Includes hashmaps for frequently used types
pub struct BasicState {
    // Boolean flags
    flags: HashMap<String, bool>,
    // Integer values
    values: HashMap<String, i32>,
    // Current scenario name
    current: String,
    // Scenario loader
    loader: String
}

pub trait State {
    /// Create a new state instance
    fn new() -> Self where Self: Sized;

    /// Reinitializes the state instance
    fn clear(&mut self);

    /// Specify the new scenario to load
    fn set_scenario(&mut self, name: String);

    /// Check if a new scenario must be loaded
    fn has_next_scenario(&self) -> bool;

    /// Obtain current scenario name
    fn get_current_scenario(&self) -> String;

    /// Get scenario to load
    fn get_next_scenario(&self) -> String;

    /// Set current scenario (on scenario change)
    fn load_scenario(&mut self);
}

impl State for BasicState {
    /// # Examples
    ///
    /// ```
    /// use texture::state::State;
    /// use texture::state::BasicState;
    ///
    /// let mut state = BasicState::new();
    /// ```
    fn new() -> BasicState {
        BasicState {
            flags: HashMap::new(),
            values: HashMap::new(),
            current: "".to_string(),
            loader: "start".to_string()
        }
    }

    /// Simply removes all keys from the internal hashmaps
    /// # Examples
    ///
    /// ```
    /// use texture::state::State;
    /// use texture::state::BasicState;
    ///
    /// let mut state = BasicState::new();
    ///
    /// // Do operations over state...
    ///
    /// // Clear state
    /// state.clear();
    /// ```
    fn clear(&mut self) {
        self.flags.clear();
        self.values.clear();
        self.current = "".to_string();
        self.loader = "start".to_string();
    }

    /// # Examples
    ///
    /// ```
    /// use texture::state::State;
    /// use texture::state::BasicState;
    ///
    /// let mut state = BasicState::new();
    ///
    /// // Set next scenario ("test_1")
    /// state.set_scenario("test_1".to_string());
    /// ```
    fn set_scenario(&mut self, name: String) {
        self.loader = name;
    }

    /// # Examples
    ///
    /// ```
    /// use texture::state::State;
    /// use texture::state::BasicState;
    ///
    /// let mut state = BasicState::new();
    ///
    /// // Check for next scenario (default is "start")
    /// assert_eq!(state.has_next_scenario(), true);
    ///
    /// // Load scenario
    /// state.load_scenario();
    ///
    /// assert_eq!(state.has_next_scenario(), false);
    /// ```
    fn has_next_scenario(&self) -> bool {
        return !self.loader.is_empty();
    }

    /// # Examples
    ///
    /// ```
    /// use texture::state::State;
    /// use texture::state::BasicState;
    ///
    /// let mut state = BasicState::new();
    ///
    /// // Get current scenario name (on startup is empty)
    /// assert_eq!(state.get_current_scenario(), "");
    ///
    /// // Load scenario ("start")
    /// state.load_scenario();
    ///
    /// assert_eq!(state.get_current_scenario(), "start");
    /// ```
    fn get_current_scenario(&self) -> String {
        return self.current.clone();
    }

    /// # Examples
    ///
    /// ```
    /// use texture::state::State;
    /// use texture::state::BasicState;
    ///
    /// let mut state = BasicState::new();
    ///
    /// // Get next scenario name (on startup is "start")
    /// assert_eq!(state.get_next_scenario(), "start");
    /// ```
    fn get_next_scenario(&self) -> String {
        return self.loader.clone();
    }

    /// # Examples
    ///
    /// ```
    /// use texture::state::State;
    /// use texture::state::BasicState;
    ///
    /// let mut state = BasicState::new();
    ///
    /// // Load the next scenario ("start")
    /// state.load_scenario();
    ///
    /// assert_eq!(state.get_current_scenario(), "start");
    ///
    /// // Load another scenario
    /// state.set_scenario("test_1".to_string());
    /// state.load_scenario();
    ///
    /// assert_eq!(state.get_current_scenario(), "test_1");
    /// ```
    fn load_scenario(&mut self) {
        self.current = self.loader.clone();
        self.loader = "".to_string();
    }
}
