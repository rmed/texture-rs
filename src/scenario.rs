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

use std::cell::RefCell;
use std::rc::Rc;

/// Scenario loader
pub struct Loader<S> {
    pub scenario: Option<Rc<RefCell<Scenario<S>>>>
}

impl <S> Loader<S> {
    /// Initialize empty loader
    pub fn new() -> Loader<S> {
        Loader { scenario: None }
    }

    /// Get scenario to load
    pub fn get_scenario(&self) -> Rc<RefCell<Scenario<S>>> {
        self.scenario.clone().unwrap().clone()
    }

    /// Set scenario to load
    pub fn set_scenario(&mut self, scenario: Rc<RefCell<Scenario<S>>>) {
        self.scenario = Some(scenario.clone());
    }
}

/// Base scenario trait
pub trait Scenario<S> {
    /// Method executed when a scenario is loaded.
    ///
    /// Ideally, only prints text describing the scenario, apart from any
    /// additional functionalities that may be included
    /// (e.g. setting flag values).
    fn load(&self, state: &Rc<RefCell<S>>, loader: &Rc<RefCell<Loader<S>>>)
            -> i32;

    /// Method executed when user input is received
    fn do_action(&self, command: &str, state: &Rc<RefCell<S>>,
                 loader: &Rc<RefCell<Loader<S>>>)
                 -> i32;
}
