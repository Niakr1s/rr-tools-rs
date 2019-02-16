use crate::global_stores::*;
use crate::spinner_button::SpinnerButton;
use gtk::{GtkListStoreExtManual, ListStore};
use rr_tools_lib::rrxml::parcel::Parcel;
use std::cell::RefCell;
use std::sync::mpsc::Receiver;

pub fn receive_from_todxf_button() -> glib::Continue {
    GLOBAL_FOR_TODXF_BUTTON.with(|global| {
        if let Some((ref button, ref rx)) = *global.borrow() {
            if let Ok(result) = rx.try_recv() {
                button.stop();
                match result {
                    Ok(()) => (),
                    Err(rrxml_paths) => {
                        println!("error while saving to dxf: {:?}", rrxml_paths);
                    } //todo error window or make them red
                };
            };
        };
        glib::Continue(false)
    })
}

pub fn receive_from_check_button() -> glib::Continue {
    GLOBAL_FOR_CHECK_BUTTON.with(|global| {
        if let Some((ref button, ref result_store, ref rx)) = *global.borrow() {
            if let Ok(parcels) = rx.try_recv() {
                if let Some(parcels) = parcels {
                    for parcel in parcels {
                        result_store.insert_with_values(None, &[0], &[&parcel.number]);
                    }
                }
                button.stop();
            }
        };
        glib::Continue(false)
    })
}
