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

use clap::Command;
use cxlib::default_impl::store::DataBase;
use indicatif::MultiProgress;

pub struct CmdAppContext {
    db: DataBase,
    command: Command,
    multi: MultiProgress,
}
impl CmdAppContext {
    pub fn new(db: DataBase, command: Command, multi: MultiProgress) -> Self {
        Self { db, command, multi }
    }
}
impl AsRef<DataBase> for CmdAppContext {
    fn as_ref(&self) -> &DataBase {
        &self.db
    }
}
impl AsRef<Command> for CmdAppContext {
    fn as_ref(&self) -> &Command {
        &self.command
    }
}
impl AsRef<MultiProgress> for CmdAppContext {
    fn as_ref(&self) -> &MultiProgress {
        &self.multi
    }
}
