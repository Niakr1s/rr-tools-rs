use crate::treeview_handle::*;
use gdk::enums::key;
use gdk::{Display, EventKey, ModifierType};
use gtk::{Clipboard, TreeView};

pub(crate) fn key_is_ctrl_c(key: &EventKey) -> bool {
    let keyval = key.get_keyval();
    let state = key.get_state();

    (state == ModifierType::CONTROL_MASK
        || state == (ModifierType::CONTROL_MASK | ModifierType::LOCK_MASK))
        && (keyval == key::C
            || keyval == key::c
            || keyval == key::Cyrillic_es
            || keyval == key::Cyrillic_ES)
}

pub(crate) fn results_to_clipboard(treeview: &TreeView, column: Option<i32>) {
    let clipboard = Clipboard::get_default(&Display::get_default().unwrap()).unwrap();
    let result = get_from_treeview_multiple(&treeview, column);
    let to_clipboard = result
        .iter()
        .map(|x| x.to_str().unwrap())
        .collect::<Vec<&str>>()
        .join("\n");
    clipboard.set_text(&to_clipboard);
    info!("copied to clipboard:\n{}", to_clipboard);
}
