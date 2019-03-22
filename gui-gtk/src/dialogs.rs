use gtk::*;
use std::path::PathBuf;

pub(crate) fn yes_or_no(window: Option<&gtk::Window>, s: &str) -> bool {
    let dialog = MessageDialog::new(
        window,
        DialogFlags::MODAL,
        MessageType::Info,
        ButtonsType::YesNo,
        s,
    );
    dialog.set_keep_above(true);
    let button_pressed = dialog.run();
    dialog.destroy();
    button_pressed == ResponseType::Yes.into()
}

pub(crate) fn error_window(window: Option<&gtk::Window>, s: &str) {
    let dialog = MessageDialog::new(
        window,
        DialogFlags::MODAL,
        MessageType::Error,
        ButtonsType::Ok,
        s,
    );
    dialog.set_keep_above(true);
    dialog.run();
    dialog.destroy();
}

pub(crate) fn choose_file(window: Option<&gtk::Window>, s: &str) -> Option<PathBuf> {
    let dialog = FileChooserDialog::with_buttons(
        Some(s),
        window,
        FileChooserAction::Save,
        &[
            ("_Cancel", ResponseType::Cancel),
            ("_Open", ResponseType::Accept),
        ],
    );
    dialog.set_keep_above(true);

    let filter = FileFilter::new();
    filter.add_pattern("*.dxf");
    dialog.set_filter(&filter);
    dialog.set_current_name("merged.dxf");

    let button_pressed = dialog.run();
    let path = dialog.get_filename();
    info!("Got {:?} from FileChooserDialog", path);
    dialog.destroy();
    if button_pressed == ResponseType::Accept.into() {
        path
    } else {
        None
    }
}
