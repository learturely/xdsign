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

use indicatif::{MultiProgress, ProgressBar};
use log::error;
use std::{error::Error as ErrorTrait, ops::Deref};
use wnewtype::NewType;
use xddcc::{ProgressState, ProgressTracker, ProgressTrackerHolder};

pub(crate) fn prog_init_error_handler<T>(e: impl ErrorTrait) -> T {
    error!("json 解析出错！错误信息：{e}.");
    panic!()
}

#[derive(Debug, NewType)]
pub struct MyProgressBar(ProgressBar);

impl ProgressTracker for MyProgressBar {
    fn inc(&self, delta: u64) {
        self.deref().inc(delta);
    }

    fn finish(&self, data: ProgressState) {
        let data = match data {
            ProgressState::GetRecordingLives => "获取回放地址完成。",
            ProgressState::GetLiveIds => "获取直播号完成。",
            ProgressState::GetLiveUrls => "已获取直播地址。",
            ProgressState::GetDeviceCodes => "获取设备码完成。",
            _ => "获取 Bug 完成。",
        };
        self.finish_with_message(data);
    }
}

#[derive(Debug, NewType)]
pub struct MyMultiProgress(MultiProgress);
impl ProgressTrackerHolder<MyProgressBar> for MultiProgress {
    fn init(&self, total: u64, data: ProgressState) -> MyProgressBar {
        let data = match data {
            ProgressState::GetRecordingLives => {
                "获取回放地址：[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}"
            }
            ProgressState::GetLiveIds => {
                "获取直播号：[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}"
            }
            ProgressState::GetLiveUrls => {
                "获取地址中：[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}"
            }
            ProgressState::GetDeviceCodes => {
                "获取设备码：[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}"
            }
            _ => "获取 Bug 中：[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        };
        let sty =
            indicatif::ProgressStyle::with_template(data).unwrap_or_else(prog_init_error_handler);
        let pb = self.add(ProgressBar::new(total));
        pb.set_style(sty);
        pb.into()
    }

    fn remove_progress(&self, progress: &MyProgressBar) {
        self.remove(progress);
    }
}
