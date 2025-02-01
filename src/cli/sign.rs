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

use crate::location_info_getter::XdsignLocationInfoGetter;
use clap::{ArgMatches, FromArgMatches};
use cxlib::{default_impl::store::DataBase, AppTrait, CmdApp, CmdMetaAppTrait, SignParser};
use log::{error, warn};

pub struct SignMainApp;
impl Default for SignMainApp {
    fn default() -> SignMainApp {
        SignMainApp
    }
}
impl<Context: AsRef<DataBase>> AppTrait<Context> for SignMainApp {
    type OwnedData = SignParser;

    fn run(&self, db: &Context, data: Self::OwnedData) {
        warn!("{}", SignParser::NOTICE);
        data.do_sign(db.as_ref(), XdsignLocationInfoGetter)
            .unwrap_or_else(|e| error!("签到失败！错误信息：{e}."));
    }
}

impl<Context: AsRef<DataBase> + 'static> CmdMetaAppTrait<CmdApp<Context>, Context> for SignMainApp
where
    Self: AppTrait<Context, OwnedData = SignParser>,
{
    fn read_owned_data(
        &self,
        _: &Context,
        matches: &[&ArgMatches],
    ) -> <Self as AppTrait<Context, ()>>::OwnedData {
        SignParser::from_arg_matches(matches.last().unwrap()).unwrap()
    }
}
