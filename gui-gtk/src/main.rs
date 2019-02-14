extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate url;

extern crate rr_tools_lib;

use rr_tools_lib::check_mydxf_in_rrxmls;
use rr_tools_lib::mydxf::MyDxf;
use rr_tools_lib::rrxml::{Parcel, RrXml};

use gdk::{Display, DragAction, EventKey, ModifierType};

use gtk::prelude::*;
use gtk::*;

use gdk::enums::key;

use url::Url;

use std::cell::RefCell;
use std::ffi::OsStr;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

// upgrade weak reference or return
#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}

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
    let rename_button: Button = builder.get_object("rename_button").expect("bad glade file");
    let check_button: Button = builder.get_object("check_button").expect("bad glade file");
    let check_button_spinner: Spinner = builder
        .get_object("check_button_spinner")
        .expect("bad glade file");

    window.set_keep_above(true);

    treeview_connect_with_drag_data_filtered(&rrxml_treeview, &rrxml_store, "xml");
    treeview_connect_with_drag_data_filtered(&mydxf_treeview, &mydxf_store, "dxf");

    treeview_connect_key_press(&rrxml_treeview, &rrxml_store);
    treeview_connect_key_press(&mydxf_treeview, &mydxf_store);

    rename_button.connect_clicked(clone!(rrxml_treeview, rrxml_store => move |_| {
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
    }));

    check_button.connect_clicked(
        clone!(rrxml_treeview, mydxf_treeview, result_store, check_button_spinner => move |_| {
            // let rrxml_paths = get_from_treeview_multiple(&rrxml_treeview);
            let rrxml_paths = get_from_treeview_all(&rrxml_treeview);
            let mydxf_path = match get_from_treeview_single(&mydxf_treeview) {
                Some(path) => path,
                None => return,
            };

            result_store.clear();
            check_button_spinner.start();

            let (tx, rx) = mpsc::channel();
            GLOBAL_RESULTSTORE.with(clone!(check_button_spinner, result_store => move |global| {
                *global.borrow_mut() = Some((check_button_spinner, result_store, rx))
            }));
            let _handle = thread::spawn(move || {
                let mydxf = MyDxf::from_file(&mydxf_path).expect("mydxf wrong path");
                let rrxmls = rrxml_paths.iter().map(|path| {
                    RrXml::from_file(&path).expect("rrxml wrong path")  //todo underlining in treeview with red color etc
                }).collect::<Vec<RrXml>>();
                let parcels = check_mydxf_in_rrxmls(&mydxf, rrxmls);
                thread::sleep(Duration::from_secs(5));  //todo remove it, just for test
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

thread_local!(
    static GLOBAL_RESULTSTORE: RefCell<
        Option<(Spinner, ListStore, Receiver<Option<Vec<Parcel>>>)>,
    > = RefCell::new(None);
);

fn global_resultstore_receive() -> glib::Continue {
    GLOBAL_RESULTSTORE.with(|global| {
        println!("{:?}", *global.borrow());
        if let Some((ref spinner, ref result_store, ref rx)) = *global.borrow() {
            if let Ok(parcels) = rx.try_recv() {
                if let Some(parcels) = parcels {
                    for parcel in parcels {
                        result_store.insert_with_values(None, &[0], &[&parcel.number]);
                    }
                    spinner.stop();
                }
            }
        };
        glib::Continue(false)
    })
}

fn get_from_treeview_single(treeview: &TreeView) -> Option<String> {
    let selection = treeview.get_selection();
    if let Some((model, iter)) = selection.get_selected() {
        return Some(model.get_value(&iter, 0).get::<String>().unwrap());
    };
    None
}

fn get_from_treeview_all(treeview: &TreeView) -> Vec<String> {
    let selection = treeview.get_selection();
    selection.select_all();
    let all = get_from_treeview_multiple(&treeview);
    selection.unselect_all();
    all
}

fn get_from_treeview_multiple(treeview: &TreeView) -> Vec<String> {
    let selection = treeview.get_selection();
    let (paths, model) = selection.get_selected_rows();
    paths
        .iter()
        .map(|path| {
            let iter = model.get_iter(path).unwrap();
            model.get_value(&iter, 0).get::<String>().unwrap()
        })
        .collect::<Vec<String>>()
}

// common for rrxml and mydxf views
fn treeview_connect_key_press(treeview: &TreeView, store: &ListStore) {
    treeview.connect_key_press_event(clone!(treeview, store => move |_, key| {
        // if event_key
        let keyval = key.get_keyval();

        if keyval == key::Delete {
            let selection = treeview.get_selection();
            let (paths, model) = selection.get_selected_rows();
            for path in paths {
                let iter = model.get_iter(&path).unwrap();
                store.remove(&iter);
            }
        };

        Inhibit(false)
    }));
}

// common for rrxml and mydxf views
fn treeview_connect_with_drag_data_filtered(
    treeview: &TreeView,
    store: &ListStore,
    filter: &'static str,
) {
    let targets = vec![gtk::TargetEntry::new(
        "text/uri-list",
        TargetFlags::OTHER_APP,
        0,
    )];
    treeview.drag_dest_set(DestDefaults::ALL, &targets, DragAction::COPY);
    treeview.connect_drag_data_received(clone!( store => move |_w, _, _, _, d, _, _| {
        let accepted_ext = Some(OsStr::new(filter));
        for file in d.get_uris() {
            let url = Url::parse(&file).expect("bad uri");
            let path = url.to_file_path().unwrap();
            if !(path.extension() == accepted_ext) {
                continue;
            };
            println!("got {:?}", path);
            let path = path.to_str().unwrap();
            store.insert_with_values(None, &[0], &[&path]);
        }
    }));
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
