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
 * Module regular_interval.
 *
 * Here are defined the RegularInterval structure and its methods.
 */

use crate::configurations::ConfigurationReader;
use super::Manager;

/**
 * A manager that runs in a regular manner.
 *
 * It's useful when you want to read / export the temperature
 * every 30 seconds for example.
 */
pub struct RegularInterval<T: ConfigurationReader> {
    /// The time interval between two runs, in seconds.
    pub interval: u32,
    config: T
}

impl<T: ConfigurationReader> Manager<T> for RegularInterval<T> {
    /// Read and export the temperature every `interval` seconds.
    fn run() {
        //TODO: Read the configuration
        //      Instantiate the reader
        //      Read the temperature
        //      Instantiate the exporter(s)
        //      Export the temperature
        //      Wait `interval` seconds
        //      Repeat
    }

    /// Set the configuration reader in the `config` attribute.
    fn set_config(&mut self, config: T) {
        self.config = config;
    }
}
