extern crate dxf;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use log::LevelFilter;
use pretty_env_logger::formatted_timed_builder;

fn main() {
    formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();
}

#[cfg(test)]
mod test;