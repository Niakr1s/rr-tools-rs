#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate url;

extern crate rr_tools_lib;

mod macros;

mod callbacks;
mod global_stores;
mod spinner_button;
mod treeview_handle;

use rr_tools_lib::check_mydxf_in_rrxmls;
use rr_tools_lib::mydxf::MyDxf;
use rr_tools_lib::rrxml::RrXml;

use gdk::{Display, EventKey, ModifierType};

use gtk::prelude::*;
use gtk::{Builder, Button, Clipboard, Dialog, ListStore, TreeView};

use gdk::enums::key;

use std::sync::mpsc;
use std::thread;

use crate::callbacks::*;
use crate::global_stores::*;
use crate::spinner_button::SpinnerButton;
use crate::treeview_handle::*;

pub fn gui_run() {
    if gtk::init().is_err() {
        error!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!(r"glade\rr-tools-rs.glade");
    let builder = Builder::new_from_string(glade_src);
    let window: gtk::Window = builder.get_object("main_window").expect("bad glade file");

    let about_dialog: Dialog = builder.get_object("about_dialog").expect("bad glade file");

    let rrxml_treeview: TreeView = builder.get_object("rrxml_view").expect("bad glade file");
    let rrxml_store: ListStore = builder.get_object("rrxml_store").expect("bad glade file");
    let mydxf_treeview: TreeView = builder.get_object("mydxf_view").expect("bad glade file");
    let mydxf_store: ListStore = builder.get_object("mydxf_store").expect("bad glade file");
    let result_treeview: TreeView = builder.get_object("result_view").expect("bad glade file");
    let result_store: ListStore = builder.get_object("result_store").expect("bad glade file");

    let rename_button = SpinnerButton::new(&builder, "rename_button", "rename_button_spinner");
    let check_button = SpinnerButton::new(&builder, "check_button", "check_button_spinner");
    let todxf_button = SpinnerButton::new(&builder, "todxf_button", "todxf_button_spinner");
    let rrxml_clear_button =
        SpinnerButton::new(&builder, "rrxml_clear_button", "rrxml_clear_button_spinner");
    let mydxf_clear_button =
        SpinnerButton::new(&builder, "mydxf_clear_button", "mydxf_clear_button_spinner");
    let clipboard_button: Button = builder
        .get_object("clipboard_button")
        .expect("bad glade file");
    let about_button: Button = builder.get_object("about_button").expect("bad glade file");

    window.set_keep_above(true);

    treeview_connect_with_drag_data_filtered(&rrxml_treeview, &rrxml_store, "xml");
    treeview_connect_with_drag_data_filtered(&mydxf_treeview, &mydxf_store, "dxf");

    treeview_connect_key_press(&rrxml_treeview, &rrxml_store);
    treeview_connect_key_press(&mydxf_treeview, &mydxf_store);

    about_button.connect_clicked(clone!(about_dialog => move |_| {
        info!("about_button clicked");
        about_dialog.run();
    }));

    about_dialog.connect_response(clone!(about_dialog => move |_, response| {
        info!("about_dialog got response: {}", response);
        // GTK_RESPONSE_DELETE_EVENT or GTK_RESPONSE_CANCEL
        if response == -4 || response == -6 {
            about_dialog.hide();
        }
    }));

    rrxml_clear_button.connect_clicked(clone!(rrxml_store => move |_| {
        info!("rrxml_clear_button clicked");
        rrxml_store.clear();
        info!("rrxml store cleared");
    }));

    mydxf_clear_button.connect_clicked(clone!(mydxf_store => move |_| {
        info!("mydxf_clear_button clicked");
        mydxf_store.clear();
        info!("mydxf store cleared");
    }));

    rename_button.connect_clicked(clone!(rrxml_treeview, rrxml_store => move |w| {
        info!("rename_button clicked");
        w.set_sensitive(false);
        let rrxml_paths = get_from_treeview_all(&rrxml_treeview, None);
        rrxml_store.clear();  // couldn't find better solution, this impl seems so stupid =\
        for rrxml_path in rrxml_paths {
            let rrxml = match RrXml::from_file(&rrxml_path) {
                Ok(rr) => rr,
                Err(_) => {error!("couldn't parse rrxml file: {}", rrxml_path); continue;},
            };

            let new_filepath = match rrxml.rename_file() {
                Ok(path) => path,
                Err(_) => {error!("error while renaming file: {}", rrxml_path); continue;},
            };

            store_insert(&rrxml_store, &new_filepath);
        }
        w.set_sensitive(true);
    }));

    todxf_button.connect_clicked(clone!(todxf_button, rrxml_treeview => move |_| {
        info!("todxf_button clicked");
        todxf_button.start();

        let rrxml_paths = get_from_treeview_all(&rrxml_treeview, None);
        info!("starting to convert to dxf: {:?}", rrxml_paths);

        let (tx, rx) = mpsc::channel();

        GLOBAL_FOR_TODXF_BUTTON.with(clone!(todxf_button, rrxml_store => move |global| {
            *global.borrow_mut() = Some((todxf_button, rrxml_store, rx))
        }));
        let _handle = thread::spawn(move || {
            let mut succesful = vec![];
            for rrxml_path in rrxml_paths {
                let rrxml = match RrXml::from_file(&rrxml_path) {
                    Ok(rr) => rr,
                    Err(_) => {error!("couldn't parse rrxml file: {}", rrxml_path); continue;},
                };
                match rrxml.save_to_dxf() {
                    Ok(_) => {info!("succesfully converted to dxf: {}", rrxml_path); succesful.push(rrxml_path)},
                    Err(_) => error!("error while converting to dxf: {:?}", rrxml_path),
                };
            }
            tx.send(Ok(succesful)).unwrap();
            glib::idle_add(receive_from_todxf_button);
        });
    }));

    check_button.connect_clicked(
        clone!(rrxml_treeview, mydxf_treeview, result_store, check_button => move |_| {
            info!("check_button clicked");
            result_store.clear();

            // let rrxml_paths = get_from_treeview_multiple(&rrxml_treeview);
            let rrxml_paths = get_from_treeview_all(&rrxml_treeview, None);
            let mydxf_path = match get_from_treeview_single(&mydxf_treeview, None) {
                Some(path) => path,
                None => return,
            };
            info!("starting check: {}", mydxf_path);

            let mydxf = match MyDxf::from_file(&mydxf_path) {
                    Ok(file) => file,
                    Err(_) => {error!("error while opening dxf file: {}", mydxf_path); return;},
                };

            check_button.start();

            let (tx, rx) = mpsc::channel();
            GLOBAL_FOR_CHECK_BUTTON.with(clone!(check_button, result_store => move |global| {
                *global.borrow_mut() = Some((check_button, result_store, rx))
            }));
            let _handle = thread::spawn(move || {
                let mut rrxmls = vec![];
                for rrxml in rrxml_paths {
                        match RrXml::from_file(&rrxml) {
                            Ok(file) => rrxmls.push(file),
                            Err(_) => {error!("error while opening xml file: {}", rrxml)},
                    }
                }
                let parcels = check_mydxf_in_rrxmls(&mydxf, rrxmls);
                tx.send(parcels).unwrap();
                glib::idle_add(receive_from_check_button);
            });
        }),
    );

    // result to clipboard
    result_treeview.connect_key_press_event(clone!(
        result_treeview => move |_,key| {
        info!("result_treeview key_press_event: {:?}", key);
        if key_is_ctrl_c(&key) {
            results_to_clipboard(&result_treeview, Some(1));
        };

        Inhibit(false)
    }));

    clipboard_button.connect_clicked(clone!(
        result_treeview => move |_| {
        info!("clipboard_button clicked");
        results_to_clipboard(&result_treeview, Some(1));
    }));

    window.show_all();

    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });

    gtk::main();
}

fn key_is_ctrl_c(key: &EventKey) -> bool {
    let keyval = key.get_keyval();
    let state = key.get_state();

    (state == ModifierType::CONTROL_MASK
        || state == (ModifierType::CONTROL_MASK | ModifierType::LOCK_MASK))
        && (keyval == key::C
            || keyval == key::c
            || keyval == key::Cyrillic_es
            || keyval == key::Cyrillic_ES)
}

fn results_to_clipboard(treeview: &TreeView, column: Option<i32>) {
    let clipboard = Clipboard::get_default(&Display::get_default().unwrap()).unwrap();
    let results = get_from_treeview_multiple(&treeview, column);
    let to_clipboard = results.join("\n");
    clipboard.set_text(&to_clipboard);
    info!("copied to clipboard:\n{}", to_clipboard);
}
