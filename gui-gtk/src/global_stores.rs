use crate::spinner_button::SpinnerButton;
use gtk::{ListStore, Window};
use rr_tools_lib::rrxml::parcel::Parcel;
use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

type Checks = Option<Vec<Parcel>>;
type Dxfs = Result<Vec<PathBuf>, ()>;
type Merged = Result<(), PathBuf>;

thread_local!(
    pub static GLOBAL_FOR_CHECK_BUTTON: RefCell<
        Option<(SpinnerButton, ListStore, Receiver<Checks>)>,
    > = RefCell::new(None);
    pub static GLOBAL_FOR_TODXF_BUTTON: RefCell<
        Option<(SpinnerButton, ListStore, Window, Receiver<(Dxfs, Merged)>)>,
    > = RefCell::new(None);
);
