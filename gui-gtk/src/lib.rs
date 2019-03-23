#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate gdk;
extern crate glib;
extern crate gtk;

extern crate url;
extern crate url_open;
use url::Url;
use url_open::UrlOpen;

extern crate rr_tools_lib;

mod dialogs;
mod macros;
mod message;
mod spinner_button;
mod treeview_handle;
mod utility;

use rr_tools_lib::check_mydxf_in_rrxml;
use rr_tools_lib::mydxf::MyDxf;
use rr_tools_lib::rrxml::{RrXml, RrXmls};

use gtk::prelude::*;
use gtk::{Builder, Button, AboutDialog, DrawingArea, Label, ListStore, ResponseType, TreeView};

use std::thread;

use crate::dialogs::*;
use crate::message::*;
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
    let window: gtk::Window = builder
        .get_object("main_window")
        .expect("no main_window in glade file");

    let about_dialog: AboutDialog = builder
        .get_object("about_dialog")
        .expect("no about_dialog in glade file");

    let rrxml_treeview: TreeView = builder.get_object("rrxml_view").expect("no  in glade file");
    let rrxml_store: ListStore = builder
        .get_object("rrxml_store")
        .expect("no rrxml_store in glade file");
    let mydxf_treeview: TreeView = builder.get_object("mydxf_view").expect("no  in glade file");
    let mydxf_store: ListStore = builder
        .get_object("mydxf_store")
        .expect("no mydxf_store in glade file");
    let result_treeview: TreeView = builder
        .get_object("result_view")
        .expect("no result_view in glade file");
    let result_store: ListStore = builder
        .get_object("result_store")
        .expect("no result_store in glade file");
    let clipboard_button: Button = builder
        .get_object("clipboard_button")
        .expect("no clipboard_button in glade file");
    let about_button: Button = builder
        .get_object("about_button")
        .expect("no about_button in glade file");
    let drawing_area: DrawingArea = builder
        .get_object("drawing_area")
        .expect("no drawing_area in glade file");
    let status_label: Label = builder
        .get_object("status_label")
        .expect("no status_label in glade file");

    // SpinnerButtons
    let rename_button = SpinnerButton::new(&builder, "rename_button", "rename_button_spinner");
    let check_button = SpinnerButton::new(&builder, "check_button", "check_button_spinner");
    let todxf_button = SpinnerButton::new(&builder, "todxf_button", "todxf_button_spinner");
    let rrxml_clear_button =
        SpinnerButton::new(&builder, "rrxml_clear_button", "rrxml_clear_button_spinner");
    let mydxf_clear_button =
        SpinnerButton::new(&builder, "mydxf_clear_button", "mydxf_clear_button_spinner");

    window.set_keep_above(true);

    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    treeview_connect_with_drag_data_filtered(&rrxml_treeview, &rrxml_store, "xml");
    treeview_connect_with_drag_data_filtered(&mydxf_treeview, &mydxf_store, "dxf");

    treeview_connect_key_press(&rrxml_treeview, &rrxml_store);
    treeview_connect_key_press(&mydxf_treeview, &mydxf_store);

    about_dialog.connect_activate_link(|_, link| {
        Url::parse(link).unwrap().open();
        Inhibit(true)
    });

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

    rename_button.connect_clicked(clone!(rrxml_treeview, rrxml_store, sender => move |w| {
        info!("rename_button clicked");

        let rrxml_paths = get_from_treeview_all(&rrxml_treeview, None);
        if rrxml_paths.is_empty() {return};

        w.set_sensitive(false);
        rrxml_store.clear();  // couldn't find better solution, this impl seems so stupid =\

        let rrxmls = RrXmls::from_files(rrxml_paths);
        for rrxml in rrxmls.rrxmls {
            let new_filepath = match rrxml.rename_file() {
                Ok(path) => path,
                Err(_) => {error!("error while renaming file: {:?}", rrxml.path); continue;},
            };
            store_insert(&rrxml_store, new_filepath.to_str().unwrap());
        }
        sender.send(Message::UpdateLabel("Успешно переименовал xml файлы".to_owned())).unwrap();
        w.set_sensitive(true);
    }));

    todxf_button.connect_clicked(clone!(todxf_button, rrxml_treeview, window, sender => move |_| {
        info!("todxf_button clicked");
        let rrxml_paths = get_from_treeview_all(&rrxml_treeview, None);
        if rrxml_paths.is_empty() {return};

        todxf_button.start();
        info!("starting to convert to dxf: {:?}", rrxml_paths);

        let merge_or = yes_or_no(Some(&window), "Объединить в один dxf?");
        let merged_path = if merge_or { choose_file(Some(&window), "Укажите путь для объединенного dxf?")} else {None};

        thread::spawn(clone!(sender => move || {
            let mut succesful = vec![];
            let rrxmls = RrXmls::from_files(rrxml_paths);
            for rrxml in &rrxmls.rrxmls {
                sender.send(Message::UpdateLabel(format!("Конвертирую {} в dxf", rrxml.path.to_str().unwrap()))).unwrap();
                match rrxml.save_to_dxf() {
                    Ok(_) => {info!("succesfully converted to dxf: {:?}", rrxml.path); succesful.push(rrxml.path.clone())},
                    Err(_) => error!("error while converting to dxf: {:?}", rrxml.path),
                };
            }
            sender.send(Message::UpdateLabel("Успешно сконвертировал xml файлы в dxf".to_owned())).unwrap();
            let merged = match merged_path {
                Some(ref p) => {
                    sender.send(Message::UpdateLabel(format!("Объединяю в один dxf {}", p.to_str().unwrap()))).unwrap();
                    if rrxmls.save_to_dxf(p.clone()).is_err() {
                        error!("error while merging to {:?}", p);
                        sender.send(Message::UpdateLabel(format!("Не удалось объединить в {}", p.to_str().unwrap()))).unwrap();
                        Some(Err(p.to_owned()))
                    } else {
                        sender.send(Message::UpdateLabel(format!("Успешно объединил в {}", p.to_str().unwrap()))).unwrap();
                        Some(Ok(p.to_owned()))
                    }
                },
                None => None,
            };
            sender.send(Message::ToDxfCompleted(succesful,merged)).unwrap();
        }));
    }));

    check_button.connect_clicked(
        clone!(rrxml_treeview, mydxf_treeview, result_store, result_treeview, check_button, sender => move |_| {
            info!("check_button clicked");

            let rrxml_paths = get_from_treeview_all(&rrxml_treeview, None);
            if rrxml_paths.is_empty() {return};
            let mydxf_path = match get_from_treeview_single(&mydxf_treeview, None) {
                Some(path) => path,
                None => return,
            };
            
            result_store.clear();
            info!("starting check: {:?}", mydxf_path);
            result_treeview.get_column(0).unwrap().set_title(&format!("{}", mydxf_path.file_name().unwrap().to_str().unwrap()));

            let mydxf = match MyDxf::from_file(mydxf_path.clone()) {
                    Ok(file) => file,
                    Err(_) => {error!("error while opening dxf file: {:?}", mydxf_path); return;},
                };

            check_button.start();

            thread::spawn(clone!(sender => move || {
                let mut rrxmls = vec![];
                for rrxml in rrxml_paths {
                        match RrXml::from_file(rrxml.clone()) {
                            Ok(file) => rrxmls.push(file),
                            Err(_) => {error!("error while opening xml file: {:?}", rrxml)},
                    }
                }
                let mut parcels = vec![];
                for rrxml in rrxmls {
                    sender.send(Message::UpdateLabel(format!("Проверяю в {}", rrxml.path.to_str().unwrap()))).unwrap();
                    if let Some(ref mut parcel) = check_mydxf_in_rrxml(&mydxf, &rrxml) {
                        parcels.append(parcel);
                    }
                }
                sender.send(Message::UpdateLabel(format!("Нашёл вхождений: {} шт для {}", parcels.len(), mydxf.path.to_str().unwrap()))).unwrap();
                sender.send(Message::CheckCompleted(parcels)).unwrap();
            }));
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

    // glib channel loop
    {
        clone_all!(
            window,
            status_label,
            rrxml_store,
            todxf_button,
            result_store,
            check_button
        );
        receiver.attach(None, move |msg| {
            match msg {
                Message::UpdateLabel(text) => {
                    status_label.set_text(&text);
                    println!("Got UpdateLabel: {}", text)
                }
                Message::CheckCompleted(parcels) => {
                    check_button.stop();
                    result_store.clear();
                    info!("succesfully checked mydxf: got {} parcels", parcels.len());
                    for parcel in parcels {
                        result_store.insert_with_values(
                            None,
                            &[0, 1],
                            &[&parcel.typ, &parcel.number],
                        );
                    }
                }
                Message::ToDxfCompleted(rrxmls, merged) => {
                    todxf_button.stop();
                    rrxml_store.clear();
                    for rrxml in rrxmls {
                        store_insert(&rrxml_store, rrxml.to_str().unwrap());
                    }
                    if let Some(res) = merged {
                        if let Err(e) = res {
                            error_window(
                                Some(&window),
                                &format!("Не удалось объединить в один dxf. Возможно, \n{}\nоткрыт в другой программе.", e.to_str().unwrap()),
                            );
                        } else {
                            info!("Merge succesful");
                        }
                    }
                }
            }
            glib::Continue(true)
        });
    }

    gtk::main();
}
