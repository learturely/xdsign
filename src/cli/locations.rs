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

use clap::{arg, ArgMatches, FromArgMatches, Parser};
use cxlib::{AppTrait, CmdMetaAppTrait};

#[derive(Parser, Debug, Clone)]
#[command(name = "locations", alias = "lsl")]
///列出所有位置。
pub struct LocationsParser {
    /// 以更好的格式显示结果。
    #[arg(short, long)]
    pretty: bool,
    /// 精简显示结果。
    #[arg(short, long)]
    short: bool,
}
#[derive(Debug, Clone, Default)]
pub struct LocationsCmdApp;

impl<Context> AppTrait<Context> for LocationsCmdApp {
    type OwnedData = LocationsParser;

    fn run(&self, _: &Context, LocationsParser { pretty, short }: Self::OwnedData) {
        if short {
            let locations = xdsign_data::LOCATIONS.iter();
            for (_, location) in locations {
                println!("{}", location,)
            }
        } else {
            // 列出所有位置。
            let locations = xdsign_data::LOCATIONS.iter();
            if pretty {
                for (alias, location) in locations {
                    println!("位置: {}, 别名: {}", location, alias)
                }
            } else {
                for (alias, location) in locations {
                    println!("{}${}", location, alias)
                }
            }
        }
    }
}
impl<Context: 'static, OwnedData: 'static> CmdMetaAppTrait<Context, OwnedData> for LocationsCmdApp {
    fn read_owned_data(
        &self,
        _: &Context,
        matches: &[&ArgMatches],
    ) -> <Self as AppTrait<Context, ()>>::OwnedData {
        let matches = matches.last().unwrap();
        LocationsParser::from_arg_matches(matches).unwrap()
    }
}
