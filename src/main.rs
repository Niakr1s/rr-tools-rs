#[allow(unused_imports)]
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use log::LevelFilter;
use pretty_env_logger::formatted_timed_builder;

mod geometry;
mod mydxf;
mod rrxml;
mod scripts;

fn main() {
    formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let rr = rrxml::RrXml::from_file("test_xmls/KPT CadastralBlock 77 03 0009007.xml");
    println!();
    println!();
    println!();
    println!();
    println!();
    let rr = rrxml::RrXml::from_file("test_xmls/KVZU Parcel 21 01 010206 115.xml");
    // println!("{:#?}", rr);
}
