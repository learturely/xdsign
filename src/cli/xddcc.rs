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
use cxlib::{
    default_impl::{store::AccountTable, store::DataBase},
    AppTrait, CmdApp, CmdMetaAppTrait,
};
use indicatif::MultiProgress;
use log::warn;
use std::path::PathBuf;
use xddcc::{lesson::Lesson, Live, PairVec, Room};

#[derive(Parser, Debug, Clone)]
#[command(name = "xddcc", alias = "x")]
/// 获取直播信息。
pub struct XddccParser {
    /// 获取特定账号下节课的直播信息，格式为以半角逗号隔开的字符串。
    #[arg(short, long)]
    pub uid: Option<String>,
    /// 通过 `device_code` 获取直播信息。
    #[arg(short, long)]
    pub device_code: Option<String>,
    /// 获取回放信息，参数为 `live_id`, 默认获取整门课的。
    #[arg(short, long)]
    pub id: Option<i64>,
    /// 获取某门课中每节课的 `live_id`.
    #[arg(short, long)]
    pub just_id: bool,
    /// 导出文件路径。可选提供。
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    /// 列出所有设备码。
    #[arg(short, long)]
    pub list: bool,
    /// 获取直播信息时覆盖默认行为至获取上节课的直播信息，获取回放信息时指定只获取一节课的回放信息。
    #[arg(short, long)]
    pub previous: bool,
    // /// 网页播放器地址。
    // #[arg(short, long)]
    // web: bool,
}
impl XddccParser {
    pub fn xddcc(self, db: &DataBase, multi_progress: &MultiProgress) {
        let XddccParser {
            uid,
            device_code,
            id,
            just_id,
            output,
            list,
            previous,
        } = self;
        if list {
            if device_code.is_some() {
                warn!("多余的参数: `-d, --device-code`.")
            }
            if previous {
                warn!("多余的参数: `-p, --previous`.")
            }
            if id.is_some() {
                warn!("多余的参数: `-r, --record`.")
            }
            if just_id {
                warn!("多余的参数: `-j, --just_id`.")
            }
            // if web {
            //     warn!("多余的参数: `-w, --web`.")
            // }
            let sessions = if let Some(uid_list_str) = uid {
                AccountTable::get_sessions_by_uid_list_str(db, &uid_list_str)
            } else {
                AccountTable::get_sessions(db)
            };
            if sessions.is_empty() {
                warn!("请至少登录一个账号！");
            }
            let rooms =
                xddcc::map_sort_by_key(Room::get_all_rooms(sessions.values(), multi_progress));
            xddcc::out(&PairVec::new(rooms), output)
        } else if let Some(device_code) = device_code {
            if uid.is_some() {
                warn!("多余的参数: `-a, --accounts`.")
            }
            if previous {
                warn!("多余的参数: `-p, --previous`.")
            }
            if id.is_some() {
                warn!("多余的参数: `-r, --record`.")
            }
            if just_id {
                warn!("多余的参数: `-j, --just_id`.")
            }
            let sessions = AccountTable::get_sessions(db);
            if sessions.is_empty() {
                warn!("未有登录的账号！");
            }
            if let Some(session) = sessions.values().next() {
                xddcc::get_live_video_path(session, &device_code)
                    .ok()
                    .map(|path| {
                        xddcc::out(&path, output.clone());
                        true
                    });
            }
        } else if let Some(live_id) = id {
            let sessions = if let Some(uid_list_str) = uid {
                AccountTable::get_sessions_by_uid_list_str(db, &uid_list_str)
            } else {
                AccountTable::get_sessions(db)
            };
            if sessions.is_empty() {
                warn!("未有登录的账号！");
            }
            if let Some(session) = sessions.into_values().next() {
                if previous {
                    if just_id {
                        warn!("多余的参数: `-j, --just_id`.")
                    }
                    xddcc::out(
                        &Lesson::get_recording_url(&session, live_id).unwrap_or_default(),
                        output.clone(),
                    );
                } else if just_id {
                    xddcc::out(
                        &Lesson::get_all_lessons(&session, live_id).unwrap_or_default(),
                        output.clone(),
                    );
                } else {
                    xddcc::out(
                        &PairVec::new(xddcc::map_sort_by_key(
                            Lesson::get_recording_lives(&session, live_id, multi_progress)
                                .unwrap_or_default(),
                        )),
                        output.clone(),
                    );
                }
            }
        } else {
            if just_id {
                warn!("多余的参数: `-j, --just_id`.")
            }
            let sessions = if let Some(uid_list_str) = uid {
                AccountTable::get_sessions_by_uid_list_str(db, &uid_list_str)
            } else {
                AccountTable::get_sessions(db)
            };
            if sessions.is_empty() {
                warn!("未有登录的账号！");
            }
            xddcc::out(
                &PairVec::new(xddcc::map_sort_by_key(Live::get_lives_now(
                    sessions.values(),
                    previous,
                    multi_progress,
                ))),
                output.clone(),
            );
        }
    }
}
pub struct XddccCmdApp {
    command: Command,
}
impl XddccCmdApp {
    pub fn new() -> Self {
        Self {
            command: XddccParser::command(),
        }
    }
}
impl Default for XddccCmdApp {
    fn default() -> Self {
        Self::new()
    }
}
impl<Context: AsRef<DataBase> + AsRef<MultiProgress>> AppTrait<Context> for XddccCmdApp {
    type OwnedData = XddccParser;

    fn run(&self, context: &Context, owned_data: Self::OwnedData) {
        owned_data.xddcc(context.as_ref(), context.as_ref())
    }
}
impl<Context: AsRef<DataBase> + AsRef<MultiProgress> + 'static>
    CmdMetaAppTrait<CmdApp<Context>, Context> for XddccCmdApp
{
    fn subcommand(&self) -> Option<&Command> {
        Some(&self.command)
    }

    fn read_owned_data(
        &self,
        _: &Context,
        matches: &[&ArgMatches],
    ) -> <Self as AppTrait<Context, ()>>::OwnedData {
        XddccParser::from_arg_matches(matches.last().unwrap()).unwrap()
    }
}
