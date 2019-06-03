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
 * Module temperature_unit.
 * 
 * Here are defined the enum Temperature and its methods.
 */

/**
 * Represents a temperature.
 */
#[derive(Debug)]
pub enum Temperature {
    Celsius(f32),
    Fahrenheit(f32)
}

use Temperature::*;
use std::fmt;

impl Temperature {
    /// Is the temperature in Celsius?
    pub fn is_celsius(&self) -> bool {
        match &self {
            Celsius(_) => true,
            Fahrenheit(_) => false
        }
    }

    /// Is the temperature in Fahrenheit?
    pub fn is_fahrenheit(&self) -> bool {
        !self.is_celsius()
    }

    /**
     * Converts a Temperature from Celsius to Fahrenheit.
     * 
     * Note: this method creates a new temperature even if self is a Fahrenheit.
     */
    pub fn to_fahrenheit(&self) -> Temperature {
        match &self {
            Celsius(c) => Fahrenheit(c * 1.8 + 32f32),
            Fahrenheit(f) => Fahrenheit(*f)
        }
    }

    /**
     * Converts a Temperature from Fahrenheit to Celsius.
     * 
     * Note: this method creates a new temperature even if self is a Celsius.
     */
    pub fn to_celsius(&self) -> Temperature {
        match &self {
            Celsius(c) => Celsius(*c),
            Fahrenheit(f) => Celsius((f - 32f32) / 1.8)
        }
    }
}

impl fmt::Display for Temperature {
    /**
     * Displays a Tempereature.
     * 
     * It has the form "42 °C" ou "107.6 °F"
     */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match &self {
            Celsius(c) => format!("{} °C", c),
            Fahrenheit(f) => format!("{} °F", f)
        };
        write!(f, "{}", value)
    }
}