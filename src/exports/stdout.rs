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
 * Module stdout.
 *
 * Here are defined the structure Stdout and its methods.
 */

use crate::models::ExportedData;
use super::Exporter;
use std::fmt::Display;

/**
 * An exporter to the stdout.
 *
 * It's supposed to be used for debug or to use redirection in Bash.
 */
pub struct Stdout {

}

impl Stdout {
    /// Get a new Stdout.
    pub fn new() -> Self {
        Stdout{}
    }
}


impl<T: Display> Exporter<T> for Stdout {
    /// Export the data to Stdout.
    fn export(&self, data: &ExportedData<T>) {
        println!("{}", data.data);
    }
}
