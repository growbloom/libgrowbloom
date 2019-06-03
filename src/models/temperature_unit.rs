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