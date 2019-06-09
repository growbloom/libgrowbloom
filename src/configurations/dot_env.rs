//Copyright 2019 Anthony Bocci
//This file is part of Temperature.
// Temperature is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// any later version.

// Temperature is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Temperature.  If not, see <https://www.gnu.org/licenses/>.

/*!
 * Here are defined the DotEnv struct and its methods.
 */

extern crate dotenv;

use dotenv::dotenv;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::env;
use super::ConfigurationReader;

/// The DotEnv configuration reader
pub struct DotEnv {
    /// The values read from the configuration.
    pub values: HashMap<String, String>,
}

impl DotEnv {
    pub fn new() -> Self {
        DotEnv { values: HashMap::new() }
    }
}

impl ConfigurationReader for DotEnv {
    /**
     * Read the configuration of the .env and set the values in
     * the `values` attribute.
     */
    fn read_configuration(&mut self) {
        dotenv().ok();
        for (key, value) in env::vars() {
            self.values.insert(key, value);
        }
    }

    /**
     * Get a value from the configuration from its key.
     * If the return type cannot be created from string
     * None is returned.
     */
    fn get_value<T: TryFrom<String>>(&self, key: &str) -> Option<T> {
        if let Some(value) = self.values.get(key) {
            match value.clone().try_into() {
                Ok(result) => return Some(result),
                Err(_) => return None,
            }
        }
        None
    }
}
