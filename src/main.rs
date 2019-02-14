#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate rr_tools_lib;

use log::LevelFilter;
use pretty_env_logger::formatted_timed_builder;

extern crate gui_gtk;
use gui_gtk::gui_run;

fn log_init() {
    formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();
}

fn main() {
    log_init();
    gui_run();
}
