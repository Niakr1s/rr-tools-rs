#[allow(unused_imports)]
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use log::LevelFilter;
use pretty_env_logger::formatted_timed_builder;

mod rr_xml;
mod scripts;

fn main() {
    formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let rr = rr_xml::RrXml::from_file("KPT CadastralBlock 77 03 0009007.xml");
    println!();
    println!();
    println!();
    println!();
    println!();
    let rr = rr_xml::RrXml::from_file("KVZU Parcel 21 01 010206 115.xml");
    // println!("{:#?}", rr);
}
