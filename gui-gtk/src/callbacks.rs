use crate::global_stores::*;
use crate::treeview_handle::store_insert;
use gtk::{GtkListStoreExt, GtkListStoreExtManual};

pub fn receive_from_todxf_button() -> glib::Continue {
    GLOBAL_FOR_TODXF_BUTTON.with(|global| {
        if let Some((ref button, ref store, ref rx)) = *global.borrow() {
            if let Ok(result) = rx.try_recv() {
                if let Ok(rrxmls) = result {
                    store.clear();
                    for rrxml in rrxmls {
                        store_insert(&store, rrxml.to_str().unwrap());
                    }
                }
                button.stop();
            };
        };
        glib::Continue(false)
    })
}

pub fn receive_from_check_button() -> glib::Continue {
    GLOBAL_FOR_CHECK_BUTTON.with(|global| {
        if let Some((ref button, ref result_store, ref rx)) = *global.borrow() {
            if let Ok(parcels) = rx.try_recv() {
                match parcels {
                    Some(parcels) => {
                        info!("succesfully checked mydxf: got {} parcels", parcels.len());
                        for parcel in parcels {
                            result_store.insert_with_values(
                                None,
                                &[0, 1],
                                &[&parcel.typ, &parcel.number],
                            );
                        }
                    }
                    None => info!("succesfully checked mydxf: got zero parcels"),
                }
                button.stop();
            }
        };
        glib::Continue(false)
    })
}
