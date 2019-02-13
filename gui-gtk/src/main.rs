extern crate gdk;
extern crate gtk;
extern crate url;

extern crate rr_tools_lib;

use rr_tools_lib::check_mydxf_in_rrxmls;
use rr_tools_lib::mydxf::MyDxf;
use rr_tools_lib::rrxml::RrXml;

use gdk::DragAction;

use gtk::prelude::*;
use gtk::*;

use url::Url;

use std::ffi::OsStr;

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

fn get_selected(treeview: &TreeView) -> Vec<String> {
    let selection = treeview.get_selection();
    let (treepaths, treemodel) = selection.get_selected_rows();
    treepaths
        .iter()
        .map(|treepath| {
            let iter = treemodel.get_iter(&treepath).unwrap();
            treemodel
                .get_value(&iter, 0)
                .get::<String>()
                .expect("not a string")
        })
        .collect()
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!(r"glade\rr-tools-rs.glade");
    let builder = Builder::new_from_string(glade_src);
    let window: gtk::Window = builder.get_object("main_window").unwrap();
    window.set_keep_above(true);

    let rrxml_treeview: TreeView = builder.get_object("rrxml_view").unwrap();
    let rrxml_store: ListStore = builder.get_object("rrxml_store").unwrap();
    let mydxf_treeview: TreeView = builder.get_object("mydxf_view").unwrap();
    let mydxf_store: ListStore = builder.get_object("mydxf_store").unwrap();
    let result_treeview: TreeView = builder.get_object("result_view").unwrap();
    let result_store: ListStore = builder.get_object("result_store").unwrap();

    treeview_connect_with_drag_data_filtered(&rrxml_treeview, &rrxml_store, "xml");
    treeview_connect_with_drag_data_filtered(&mydxf_treeview, &mydxf_store, "dxf");

    let rename_button: Button = builder.get_object("rename_button").unwrap();
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

    let check_button: Button = builder.get_object("check_button").unwrap();
    check_button.connect_clicked(
        clone!(rrxml_treeview, mydxf_treeview, result_store => move |_| {
            // let rrxml_paths = get_from_treeview_multiple(&rrxml_treeview);
            let rrxml_paths = get_from_treeview_all(&rrxml_treeview);
            let mydxf_path = match get_from_treeview_single(&mydxf_treeview) {
                Some(path) => path,
                None => return,
            };

            let mydxf = MyDxf::from_file(&mydxf_path).expect("mydxf wrong path");
            let rrxmls = rrxml_paths.iter().map(|path| {
                RrXml::from_file(&path).expect("rrxml wrong path")  //todo underlining in treeview with red color etc
            }).collect::<Vec<RrXml>>();

            let parcels = check_mydxf_in_rrxmls(&mydxf, rrxmls);

            result_store.clear();
            if let Some(parcels) = parcels {
                for parcel in parcels {
                    result_store.insert_with_values(None, &[0], &[&parcel.number]);
                }
            }

        }),
    );

    window.show_all();

    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });

    gtk::main();
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
    treeview.connect_drag_data_received(clone!( store => move |w, _, _, _, d, _, _| {
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
