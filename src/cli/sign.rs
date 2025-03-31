use crate::location_info_getter::XdsignLocationInfoGetter;
use clap::{ArgMatches, FromArgMatches};
use cxlib::{
    default_impl::store::DataBase, AppTrait, CmdMetaAppTrait, CourseDataFilterAndSorterTrait,
    DefaultCourseDataSorter, SignParser,
};
use log::{error, warn};
use std::marker::PhantomData;

pub struct SignMainApp<T = DefaultCourseDataSorter> {
    _t: PhantomData<T>,
}
impl<T> Default for SignMainApp<T> {
    #[inline]
    fn default() -> SignMainApp<T> {
        SignMainApp {
            _t: Default::default(),
        }
    }
}

impl<Context, T> AppTrait<Context> for SignMainApp<T>
where
    Context: AsRef<DataBase>,
    T: CourseDataFilterAndSorterTrait,
{
    type OwnedData = SignParser;

    fn run(&self, db: &Context, data: Self::OwnedData) {
        warn!("{}", SignParser::notice_content());
        data.do_sign(db.as_ref(), XdsignLocationInfoGetter, T::filter, T::sorter)
            .unwrap_or_else(|e| error!("签到失败！错误信息：{e}."));
    }
}

impl<Context, OwnedData, T> CmdMetaAppTrait<Context, OwnedData> for SignMainApp<T>
where
    Context: AsRef<DataBase> + 'static,
    OwnedData: 'static,
    T: CourseDataFilterAndSorterTrait + 'static,
{
    fn read_owned_data(
        &self,
        _: &Context,
        matches: &[&ArgMatches],
    ) -> <Self as AppTrait<Context, ()>>::OwnedData {
        SignParser::from_arg_matches(matches.last().unwrap()).unwrap()
    }
}
