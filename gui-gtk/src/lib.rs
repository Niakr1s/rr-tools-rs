#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate url;

extern crate rr_tools_lib;

mod callbacks;
mod dialogs;
mod global_stores;
mod macros;
mod message;
mod spinner_button;
mod treeview_handle;
mod utility;

use rr_tools_lib::check_mydxf_in_rrxmls;
use rr_tools_lib::mydxf::MyDxf;
use rr_tools_lib::rrxml::{RrXml, RrXmls};

use gtk::prelude::*;
use gtk::{Builder, Button, Dialog, DrawingArea, ListStore, ResponseType, TreeView};

use std::sync::mpsc;
use std::thread;

use crate::callbacks::*;
use crate::dialogs::*;
use crate::global_stores::*;
use crate::message::Message;
use crate::spinner_button::SpinnerButton;
use crate::treeview_handle::*;
use crate::utility::*;

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
    let drawing_area: DrawingArea = builder.get_object("drawing_area").expect("bad glade file");
    window.set_keep_above(true);

    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    receiver.attach(None, move |msg| {
        match msg {
            Message::UpdateLabel(text) => println!("Got UpdateLabel: {}", text),
            Message::Checked(parcels) => println!("Got checked parcels: {:?}", parcels),
            _ => (),
        }
        glib::Continue(true)
    });

    treeview_connect_with_drag_data_filtered(&rrxml_treeview, &rrxml_store, "xml");
    treeview_connect_with_drag_data_filtered(&mydxf_treeview, &mydxf_store, "dxf");

    treeview_connect_key_press(&rrxml_treeview, &rrxml_store);
    treeview_connect_key_press(&mydxf_treeview, &mydxf_store);

    drawing_area.connect_draw(clone!(drawing_area => move |_, cr| {
        // todo
        let x_h = drawing_area.get_allocated_width();
        let y_h = drawing_area.get_allocated_height();
        cr.set_source_rgb(255.0, 255.0, 255.0);
        cr.paint();
        // draw 100 random black lines
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.move_to(f64::from(x_h) * 0.5, f64::from(y_h) * 0.5);
        cr.line_to(f64::from(x_h) * 0.6, f64::from(y_h) * 0.6);
        cr.stroke();

        Inhibit(false)
    }));

    about_button.connect_clicked(clone!(about_dialog => move |_| {
        info!("about_button clicked");
        about_dialog.run();
    }));

    about_dialog.connect_response(clone!(about_dialog => move |_, response| {
        info!("about_dialog got response: {:?}", response);
        // GTK_RESPONSE_DELETE_EVENT or GTK_RESPONSE_CANCEL
        if response == ResponseType::DeleteEvent || response == ResponseType::Cancel {
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
        let rrxmls = RrXmls::from_files(rrxml_paths);
        for rrxml in rrxmls.rrxmls {
            let new_filepath = match rrxml.rename_file() {
                Ok(path) => path,
                Err(_) => {error!("error while renaming file: {:?}", rrxml.path); continue;},
            };
            store_insert(&rrxml_store, new_filepath.to_str().unwrap());
        }
        w.set_sensitive(true);
    }));

    // todxf_button.connect_clicked(clone!(todxf_button, rrxml_treeview, window => move |_| {
    //     info!("todxf_button clicked");
    //     todxf_button.start();

    //     let rrxml_paths = get_from_treeview_all(&rrxml_treeview, None);
    //     info!("starting to convert to dxf: {:?}", rrxml_paths);

    //     let (tx, rx) = mpsc::channel();

    //     GLOBAL_FOR_TODXF_BUTTON.with(clone!(todxf_button, rrxml_store, window => move |global| {
    //         *global.borrow_mut() = Some((todxf_button, rrxml_store, window, rx))
    //     }));

    //     let merge_or = yes_or_no(Some(&window), "Merge into one dxf?");
    //     let merged_path = if merge_or { choose_file(Some(&window), "Where to merge dxfs?")} else {None};

    //     thread::spawn(move || {
    //         let mut succesful = vec![];
    //         let rrxmls = RrXmls::from_files(rrxml_paths);
    //         for rrxml in &rrxmls.rrxmls {
    //             match rrxml.save_to_dxf() {
    //                 Ok(_) => {info!("succesfully converted to dxf: {:?}", rrxml.path); succesful.push(rrxml.path.clone())},
    //                 Err(_) => error!("error while converting to dxf: {:?}", rrxml.path),
    //             };
    //         }
    //         let merged_result = match merged_path {
    //             Some(p) => if rrxmls.save_to_dxf(p.clone()).is_err() {
    //                 error!("error while merging to {:?}", p);
    //                 Err(p)
    //             } else {
    //                 Ok(())
    //             },
    //             None => Ok(()),
    //         };
    //         tx.send((Ok(succesful),merged_result)).unwrap();
    //         glib::idle_add(receive_from_todxf_button);
    //     });
    // }));

    todxf_button.connect_clicked(clone!(todxf_button, rrxml_treeview, window => move |_| {
        info!("todxf_button clicked");
        todxf_button.start();

        let rrxml_paths = get_from_treeview_all(&rrxml_treeview, None);
        info!("starting to convert to dxf: {:?}", rrxml_paths);

        let (tx, rx) = mpsc::channel();

        GLOBAL_FOR_TODXF_BUTTON.with(clone!(todxf_button, rrxml_store, window => move |global| {
            *global.borrow_mut() = Some((todxf_button, rrxml_store, window, rx))
        }));

        let merge_or = yes_or_no(Some(&window), "Merge into one dxf?");
        let merged_path = if merge_or { choose_file(Some(&window), "Where to merge dxfs?")} else {None};

        thread::spawn(move || {
            let mut succesful = vec![];
            let rrxmls = RrXmls::from_files(rrxml_paths);
            for rrxml in &rrxmls.rrxmls {
                match rrxml.save_to_dxf() {
                    Ok(_) => {info!("succesfully converted to dxf: {:?}", rrxml.path); succesful.push(rrxml.path.clone())},
                    Err(_) => error!("error while converting to dxf: {:?}", rrxml.path),
                };
            }
            let merged_result = match merged_path {
                Some(p) => if rrxmls.save_to_dxf(p.clone()).is_err() {
                    error!("error while merging to {:?}", p);
                    Err(p)
                } else {
                    Ok(())
                },
                None => Ok(()),
            };
            tx.send((Ok(succesful),merged_result)).unwrap();
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
            info!("starting check: {:?}", mydxf_path);

            let mydxf = match MyDxf::from_file(mydxf_path.clone()) {
                    Ok(file) => file,
                    Err(_) => {error!("error while opening dxf file: {:?}", mydxf_path); return;},
                };

            check_button.start();

            let (tx, rx) = mpsc::channel();
            GLOBAL_FOR_CHECK_BUTTON.with(clone!(check_button, result_store => move |global| {
                *global.borrow_mut() = Some((check_button, result_store, rx))
            }));
            thread::spawn(move || {
                let mut rrxmls = vec![];
                for rrxml in rrxml_paths {
                        match RrXml::from_file(rrxml.clone()) {
                            Ok(file) => rrxmls.push(file),
                            Err(_) => {error!("error while opening xml file: {:?}", rrxml)},
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

    window.connect_delete_event(move |_win, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
