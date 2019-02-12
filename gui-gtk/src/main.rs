extern crate gdk;
extern crate gtk;
extern crate url;

use gdk::DragAction;

use gtk::prelude::*;
use gtk::*;

use url::Url;

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

    let rrxml_treeview: TreeView = builder.get_object("rrxml_view").unwrap();
    let mydxf_treeview: TreeView = builder.get_object("mydxf_view").unwrap();

    // Configure the text view to accept URI lists from other applications. This allows
    // dragging files & folders from a file browser program onto the textview.
    let targets = vec![gtk::TargetEntry::new(
        "text/uri-list",
        TargetFlags::OTHER_APP,
        0,
    )];

    rrxml_treeview.drag_dest_set(DestDefaults::ALL, &targets, DragAction::COPY);
    rrxml_treeview.connect_drag_data_received(|w, _, _, _, d, _, _| {
        println!("d&d recieved");
        for file in d.get_uris() {
            let url = Url::parse(&file).expect("bad uri");
            let path = url.to_file_path();
            println!("{:?}", path);
        }
    });

    window.show_all();

    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });

    gtk::main();
}
