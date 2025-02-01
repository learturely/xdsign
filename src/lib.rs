// Copyright (C) 2024 worksoup <https://github.com/worksoup/>
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

mod cli;
mod cmd_app_context;
mod location_info_getter;
mod my_progress_bar;
pub fn run() {
    use crate::{
        cli::{LocationsCmdApp, SignMainApp, XddccCmdApp},
        cmd_app_context::CmdAppContext,
    };
    use cxlib::{
        types::Location,
        user::{LoginSolverTrait, LoginSolvers},
        utils::time_it_and_print_result,
        AccountCmdApp, AccountsCmdApp, AppTrait, CmdApp, CoursesCmdApp, ListCmdApp,
        WhereIsConfigCmdApp,
    };
    use indicatif::MultiProgress;
    use log::{error, warn};
    use x_l4rs::IDSLoginImpl;
    use xdsign_data::LocationPreprocessor;
    fn init(self_: &CmdApp<CmdAppContext>) -> (CmdAppContext, ()) {
        use cxlib::{
            captcha::CaptchaType,
            default_impl::store::{
                AccountTable, AliasTable, DataBase, ExcludeTable, LocationTable,
            },
            store::Dir,
        };
        fn init_output() -> MultiProgress {
            let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
            let mut builder = env_logger::Builder::from_env(env);
            builder.target(env_logger::Target::Stderr);
            let logger = builder.build();
            let multi = MultiProgress::new();
            indicatif_log_bridge::LogWrapper::new(multi.clone(), logger)
                .try_init()
                .unwrap_or_else(|e| {
                    error!("日志初始化失败。错误信息：{e}.");
                    panic!()
                });
            multi
        }
        fn init_function() {
            if let Some(captcha_type) = std::env::var("CX_CAPTCHA_TYPE")
                .ok()
                .and_then(|s| s.parse().ok())
            {
                let _ = CaptchaType::set_global_default(&captcha_type);
            }
            Dir::set_config_dir_info("TEST_XDSIGN", "rt.lea", "Leart", env!("CARGO_PKG_NAME"));
            Location::set_boxed_location_preprocessor(Box::new(LocationPreprocessor))
                .unwrap_or_else(|e| error!("{e}"));
            let login_solver = IDSLoginImpl::TARGET_LEARNING.get_login_solver(|a, b| {
                time_it_and_print_result(|| {
                    use cxlib::imageproc::{
                        find_sub_image,
                        match_template::{match_template_for_slide, MatchTemplateMethod},
                    };
                    Ok(find_sub_image(a, b, |a, b, mask| {
                        match_template_for_slide(
                            a,
                            b,
                            MatchTemplateMethod::CrossCorrelationNormalized,
                            mask,
                        )
                    }))
                })
            });
            let login_type = login_solver.login_type().to_owned();
            LoginSolvers::register(login_solver)
                .unwrap_or_else(|_| warn!("登录协议 `{login_type}` 注册失败！"));
        }
        let db = DataBase::default();
        db.add_table::<AccountTable>();
        db.add_table::<ExcludeTable>();
        db.add_table::<AliasTable>();
        db.add_table::<LocationTable>();
        init_function();
        let multi = init_output();
        (CmdAppContext::new(db, self_.command().clone(), multi), ())
    }
    let cmd_app = CmdApp::new(clap::command!())
        .main_cmd_app(SignMainApp)
        .meta_app(ListCmdApp::default())
        .meta_app(AccountCmdApp::default())
        .meta_app(AccountsCmdApp::default())
        .meta_app(CoursesCmdApp::default())
        .meta_app(LocationsCmdApp::default())
        .meta_app(XddccCmdApp::default())
        .meta_app(WhereIsConfigCmdApp::default());
    #[cfg(feature = "completion")]
    let cmd_app = cmd_app.meta_app(cxlib::CompletionsCmdApp::default());
    cmd_app.init_and_run(init)
}
