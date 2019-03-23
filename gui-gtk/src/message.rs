use crate::rr_tools_lib::rrxml::parcel::Parcel;
use std::path::PathBuf;

pub type Checks = Vec<Parcel>;
pub type SuccesfullRrXmls = Vec<PathBuf>;
pub type Merged = Option<Result<PathBuf, PathBuf>>;

#[derive(Clone)]
pub enum Message {
    UpdateLabel(String),
    CheckCompleted(Checks),
    ToDxfCompleted(SuccesfullRrXmls, Merged),
}
