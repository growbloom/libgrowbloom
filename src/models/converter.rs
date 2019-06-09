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
 * Module converter.
 *
 * Here is defined the Converter trait.
 */

/**
 * The Converter trait.
 *
 * Any model must implement it to be converted into another type.
 * It might be useful to convert a Temperature to a String.
 */
pub trait Converter<T> {
    /// Converts the current instance into T.
    fn convert(&self) -> T;
}