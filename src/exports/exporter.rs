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
 * Module exporter.
 *
 * Here is defined the Exporter trait.
 */

use crate::models::{Converter, ExportedData };

///A trait that every exporter has to implement.
/// It aims to export a data to any destination
/// (database, file, webservice, ...)
pub trait Exporter<T, U>
where T: Converter<U> {
    fn export(&self, data: &ExportedData<T>);
}
