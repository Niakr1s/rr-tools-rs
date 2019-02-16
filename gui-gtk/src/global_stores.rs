use crate::spinner_button::SpinnerButton;
use gtk::{GtkListStoreExtManual, ListStore};
use rr_tools_lib::rrxml::parcel::Parcel;
use std::cell::RefCell;
use std::sync::mpsc::Receiver;

thread_local!(
    pub static GLOBAL_RESULTSTORE: RefCell<
        Option<(SpinnerButton, ListStore, Receiver<Option<Vec<Parcel>>>)>,
    > = RefCell::new(None);
);

pub fn global_resultstore_receive() -> glib::Continue {
    GLOBAL_RESULTSTORE.with(|global| {
        if let Some((ref button_with_spinner, ref result_store, ref rx)) = *global.borrow() {
            if let Ok(parcels) = rx.try_recv() {
                if let Some(parcels) = parcels {
                    for parcel in parcels {
                        result_store.insert_with_values(None, &[0], &[&parcel.number]);
                    }
                }
                button_with_spinner.stop();
            }
        };
        glib::Continue(false)
    })
}
