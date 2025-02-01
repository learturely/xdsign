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

use clap::{arg, ArgMatches, Command, CommandFactory, FromArgMatches, Parser};
use cxlib::{AppTrait, CmdApp, CmdMetaAppTrait};

#[derive(Parser, Debug, Clone)]
#[command(name = "locations")]
///列出所有位置。
pub struct LocationsParser {
    /// 以更好的格式显示结果。
    #[arg(short, long)]
    pretty: bool,
    /// 精简显示结果。
    #[arg(short, long)]
    short: bool,
}

pub struct LocationsCmdApp {
    command: Command,
}
impl Default for LocationsCmdApp {
    fn default() -> Self {
        Self::new()
    }
}

impl LocationsCmdApp {
    pub fn new() -> LocationsCmdApp {
        let command = LocationsParser::command();
        LocationsCmdApp { command }
    }
}

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
impl<Context: 'static> CmdMetaAppTrait<CmdApp<Context>, Context> for LocationsCmdApp {
    fn subcommand(&self) -> Option<&Command> {
        Some(&self.command)
    }

    fn read_owned_data(&self, _: &Context, matches: &[&ArgMatches]) -> Self::OwnedData {
        let matches = matches.last().unwrap();
        LocationsParser::from_arg_matches(matches).unwrap()
    }
}
