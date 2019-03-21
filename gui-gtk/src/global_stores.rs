use crate::spinner_button::SpinnerButton;
use gtk::ListStore;
use rr_tools_lib::rrxml::parcel::Parcel;
use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

thread_local!(
    pub static GLOBAL_FOR_CHECK_BUTTON: RefCell<
        Option<(SpinnerButton, ListStore, Receiver<Option<Vec<Parcel>>>)>,
    > = RefCell::new(None);
    pub static GLOBAL_FOR_TODXF_BUTTON: RefCell<
        Option<(SpinnerButton, ListStore, Receiver<Result<Vec<PathBuf>, ()>>)>,
    > = RefCell::new(None);
);
