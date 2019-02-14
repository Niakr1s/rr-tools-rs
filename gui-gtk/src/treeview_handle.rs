use gdk::enums::key;
use gdk::DragAction;
use gtk::*;

use url::Url;

use std::ffi::OsStr;

pub fn get_from_treeview_single(treeview: &TreeView) -> Option<String> {
    let selection = treeview.get_selection();
    if let Some((model, iter)) = selection.get_selected() {
        return Some(model.get_value(&iter, 0).get::<String>().unwrap());
    };
    None
}

pub fn get_from_treeview_all(treeview: &TreeView) -> Vec<String> {
    let selection = treeview.get_selection();
    selection.select_all();
    let all = get_from_treeview_multiple(&treeview);
    selection.unselect_all();
    all
}

pub fn get_from_treeview_multiple(treeview: &TreeView) -> Vec<String> {
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
pub fn treeview_connect_key_press(treeview: &TreeView, store: &ListStore) {
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
pub fn treeview_connect_with_drag_data_filtered(
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
