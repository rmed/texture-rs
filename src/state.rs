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
///
/// It is recommended to build a custom state type rather than using this
/// basic one.
pub struct BasicState {
    // Boolean flags
    flags: HashMap<String, bool>,
    // Integer values
    values: HashMap<String, i32>,
}

pub trait BaseState {
    /// Create a new state instance
    fn new() -> Self where Self: Sized;

    /// Reinitializes the state instance
    fn clear(&mut self);

    /// Set an internal flag
    fn set_flag(&mut self, name: String, value: bool);

    /// Obtain the value of a flag
    fn get_flag(&self, name: String) -> bool;

    /// Set an internal integer value
    fn set_value(&mut self, name: String, value: i32);

    /// Obtain an internal integer value
    fn get_value(&self, name: String) -> i32;
}

impl BaseState for BasicState {
    /// # Examples
    ///
    /// ```
    /// use texture::state::BaseState;
    /// use texture::state::BasicState;
    ///
    /// let mut state = BasicState::new();
    /// ```
    fn new() -> BasicState {
        BasicState {
            flags: HashMap::new(),
            values: HashMap::new(),
        }
    }

    /// Simply removes all keys from the internal hashmaps
    /// # Examples
    ///
    /// ```
    /// use texture::state::BaseState;
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
    }

    /// # Examples
    ///
    /// ```
    /// use texture::state::BaseState;
    /// use texture::state::BasicState;
    ///
    /// let mut state = BasicState::new();
    ///
    /// // Set flag
    /// state.set_flag("in_start".to_string(), true);
    ///
    /// assert_eq!(state.get_flag("in_start".to_string()), true);
    /// ```
    fn set_flag(&mut self, name: String, value: bool) {
        self.flags.insert(name, value);
    }

    /// # Examples
    ///
    /// ```
    /// use texture::state::BaseState;
    /// use texture::state::BasicState;
    ///
    /// let mut state = BasicState::new();
    ///
    /// // Set flag
    /// state.set_flag("in_start".to_string(), true);
    /// assert_eq!(state.get_flag("in_start".to_string()), true);
    ///
    /// state.set_flag("in_start".to_string(), false);
    /// assert_eq!(state.get_flag("in_start".to_string()), false);
    /// ```
    fn get_flag(&self, name: String) -> bool {
        let val = match self.flags.get(&name) {
            Some(s) => { s.clone() },
            None => { false }
        };

        return val;
    }

    /// # Examples
    ///
    /// ```
    /// use texture::state::BaseState;
    /// use texture::state::BasicState;
    ///
    /// let mut state = BasicState::new();
    ///
    /// // Set flag
    /// state.set_value("time".to_string(), 850);
    /// assert_eq!(state.get_value("time".to_string()), 850);
    /// ```
    fn set_value(&mut self, name: String, value: i32) {
        self.values.insert(name, value);
    }

    /// # Examples
    ///
    /// ```
    /// use texture::state::BaseState;
    /// use texture::state::BasicState;
    ///
    /// let mut state = BasicState::new();
    ///
    /// // Set flag
    /// state.set_value("time".to_string(), 850);
    /// assert_eq!(state.get_value("time".to_string()), 850);
    ///
    /// state.set_value("time".to_string(), 700);
    /// assert_eq!(state.get_value("time".to_string()), 700);
    /// ```
    fn get_value(&self, name: String) -> i32 {
        let val = match self.values.get(&name) {
            Some(s) => { s.clone() },
            None => { 0 }
        };

        return val;
    }
}
