extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate url;

extern crate rr_tools_lib;

mod macros;

mod global_stores;
mod spinner_button;
mod treeview_handle;

mod gui;
use gui::gui_run;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    gui_run();
}
