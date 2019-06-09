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
 * Module manager.
 *
 * Here is defined the Manager trait.
 */

use crate::configurations::ConfigurationReader;

pub trait Manager<T: ConfigurationReader> {
    /**
     * Run the manager.
     *
     * That's where the main of the application works, where the
     * readers and exporters should be used.
     */
    fn run();

    /**
     * Set the configuration reader.
     *
     * It's up to each manager to choose what this method should do.
     * It might store it in a field, or in database, there is no
     * requirement on that part.
     */
    fn set_config(&mut self, config: T);
}
