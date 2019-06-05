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
 * Here are defined the ConfigurationReader trait and its methods.
 */

///This trait has to be implemented by every configuration reader.
pub trait ConfigurationReader {

    /**
     * Read the configuration.
     *
     * The variables required by the managers are read here and
     * should be available then using this same configuration reader.
     */
    fn read_configuration();

    /**
     * Get a configuration value by its key.
     *
     * Should be called after read_configuration() because the configuration
     * has to be read before being able to get its values.
     * Because we can try to read an undefined value, the method returns
     * an Option.
     */
    fn get_value<T>(key: &str) -> Option<T>;
}
