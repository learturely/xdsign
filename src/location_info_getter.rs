// Copyright (C) 2025 learturely <learturely@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use cxlib::{
    default_impl::{sign::LocationSign, signner::LocationInfoGetterTrait},
    types::Location,
};
use xdsign_data::LOCATIONS;
#[derive(Copy, Clone, Debug)]
pub struct XdsignLocationInfoGetter;

impl LocationInfoGetterTrait for XdsignLocationInfoGetter {
    fn get_location_by_location_str(&self, location_str: &str) -> Option<Location> {
        LOCATIONS.get(location_str).cloned()
    }
    fn get_fallback_location(&self, _: &LocationSign) -> Option<Location> {
        LOCATIONS.values().next().cloned()
    }
}
