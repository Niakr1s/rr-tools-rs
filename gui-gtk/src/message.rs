use crate::rr_tools_lib::rrxml::parcel::Parcel;

pub enum Message {
    UpdateLabel(String),
    Checked(Vec<Parcel>),
}
