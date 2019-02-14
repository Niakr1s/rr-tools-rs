extern crate log;
extern crate pretty_env_logger;
extern crate rr_tools_lib;

use log::LevelFilter;
use pretty_env_logger::formatted_timed_builder;

fn init() {
    formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();
}

fn main() {
    init();
}
