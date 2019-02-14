extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate url;

extern crate rr_tools_lib;

mod macros;

use rr_tools_lib::check_mydxf_in_rrxmls;
use rr_tools_lib::mydxf::MyDxf;
use rr_tools_lib::rrxml::RrXml;

use gdk::{Display, EventKey, ModifierType};

use gtk::prelude::*;
use gtk::{Builder, Clipboard, ListStore, TreeView};

use gdk::enums::key;

use std::sync::mpsc;
use std::thread;

mod spinner_button;
use spinner_button::SpinnerButton;

mod treeview_handle;
use treeview_handle::*;

mod global_stores;
use global_stores::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!(r"glade\rr-tools-rs.glade");
    let builder = Builder::new_from_string(glade_src);
    let window: gtk::Window = builder.get_object("main_window").expect("bad glade file");

    let rrxml_treeview: TreeView = builder.get_object("rrxml_view").expect("bad glade file");
    let rrxml_store: ListStore = builder.get_object("rrxml_store").expect("bad glade file");
    let mydxf_treeview: TreeView = builder.get_object("mydxf_view").expect("bad glade file");
    let mydxf_store: ListStore = builder.get_object("mydxf_store").expect("bad glade file");
    let result_treeview: TreeView = builder.get_object("result_view").expect("bad glade file");
    let result_store: ListStore = builder.get_object("result_store").expect("bad glade file");

    let rename_button = SpinnerButton::new(&builder, "rename_button", "rename_button_spinner");
    let check_button = SpinnerButton::new(&builder, "check_button", "check_button_spinner");

    window.set_keep_above(true);

    treeview_connect_with_drag_data_filtered(&rrxml_treeview, &rrxml_store, "xml");
    treeview_connect_with_drag_data_filtered(&mydxf_treeview, &mydxf_store, "dxf");

    treeview_connect_key_press(&rrxml_treeview, &rrxml_store);
    treeview_connect_key_press(&mydxf_treeview, &mydxf_store);

    rename_button.connect_clicked(clone!(rrxml_treeview, rrxml_store => move |w| {
        w.set_sensitive(false);
        let selection = rrxml_treeview.get_selection();
        let (treepaths, model) = selection.get_selected_rows();

        for treepath in treepaths {
            let iter = model.get_iter(&treepath).unwrap();
            let filepath = model.get_value(&iter, 0).get::<String>().unwrap();
            println!("filepath is {:?}", filepath);
            let rrxml = RrXml::from_file(&filepath).expect("error while creating rrxml from file");
            let new_filepath = rrxml.rename_file().expect("error while renaming rrxml file");
            rrxml_store.set(&iter, &[0], &[&new_filepath.to_value()]);
        }
        w.set_sensitive(true);
    }));

    check_button.connect_clicked(
        clone!(rrxml_treeview, mydxf_treeview, result_store, check_button => move |_| {
            result_store.clear();

            // let rrxml_paths = get_from_treeview_multiple(&rrxml_treeview);
            let rrxml_paths = get_from_treeview_all(&rrxml_treeview);
            let mydxf_path = match get_from_treeview_single(&mydxf_treeview) {
                Some(path) => path,
                None => return,
            };

            check_button.start();

            let (tx, rx) = mpsc::channel();
            GLOBAL_RESULTSTORE.with(clone!(check_button, result_store => move |global| {
                *global.borrow_mut() = Some((check_button, result_store, rx))
            }));
            let _handle = thread::spawn(move || {
                let mydxf = MyDxf::from_file(&mydxf_path).expect("mydxf wrong path");
                let rrxmls = rrxml_paths.iter().map(|path| {
                    RrXml::from_file(&path).expect("rrxml wrong path")  //todo underlining in treeview with red color etc
                }).collect::<Vec<RrXml>>();
                let parcels = check_mydxf_in_rrxmls(&mydxf, rrxmls);
                println!("got parcels!");
                tx.send(parcels).unwrap();
                glib::idle_add(global_resultstore_receive);
            });
        }),
    );

    // result to clipboard
    result_treeview.connect_key_press_event(clone!(
        result_treeview => move |_,key| {
        if key_is_ctrl_c(&key) {
            println!("got ctrl+c event");
            let clipboard = Clipboard::get_default(&Display::get_default().unwrap()).unwrap();
            let results = get_from_treeview_multiple(&result_treeview);
            let to_clipboard = results.join("\n");
            clipboard.set_text(&to_clipboard);
        };

        Inhibit(false)
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
