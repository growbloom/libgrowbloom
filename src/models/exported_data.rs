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
 * Module exported_data.
 *
 * Here are defined the ExportedData structure and its methods.
 */

use super::Temperature;

/**
 * Represents a data that will be exported.
 *
 * It aims to standardize the format of exported datas whatever the
 * exporter is.
 */
#[derive(Debug)]
pub struct ExportedData {
    ///The temperature we are exporting.
    pub temperature: Temperature,
}

impl ExportedData {
    /// Get a new ExportedData.
    pub fn new(temperature: Temperature) -> Self {
        ExportedData{ temperature }
    }
}
