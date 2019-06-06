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

impl ConfigurationReader for DotEnv {
    /**
     * Read the configuration of the .env and set the values in
     * the `values` attribute.
     */
    fn read_configuration() {
        dotenv().ok();
        //TODO: set the `values` attribute.
    }

    /**
     * Get a value from the configuration from its key.
     * If the return type cannot be created from string
     * None is returned.
     */
    fn get_value<T: TryFrom<String>>(key: &str) -> Option<T> {
        // Get the last env var with the given name.
        // We get a tuple, 0 is key and 1 is value.
        let key_value: Option<(String, String)> = env::vars()
        .filter(|item| item.0 == key)
        .last();
        match key_value {
            Some(tuple) => {
                match tuple.1.try_into() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            },
            None    => None
        }
    }
}
