#[macro_use] extern crate log;

#[macro_use] pub mod geometry;
pub mod mydxf;
pub mod rrxml;
pub mod error;

use crate::mydxf::MyDxf;
use crate::rrxml::Parcel;
use crate::rrxml::RrXml;
use crate::geometry::traits::relative::Relative;

pub fn check_mydxf_in_rrxmls(mydxf: &MyDxf, rrxmls: Vec<RrXml>) -> Option<Vec<Parcel>> {
    let mut parcels = vec![];
    for rrxml in rrxmls {
        if let Some(ref mut parcel) = check_mydxf_in_rrxml(mydxf, &rrxml) {
            parcels.append(parcel);
        }
    };
    match parcels.len() {
        0 => None,
        _ => Some(parcels),
    }
}

pub fn check_mydxf_in_rrxml(mydxf: &MyDxf, rrxml: &RrXml) -> Option<Vec<Parcel>> {
    if rrxml.len() == 0 { return None };

    let mut parcels = vec![];

    for parcel in &rrxml.parcels {
        if check_mydxf_in_parcel(&mydxf, &parcel) {
            parcels.push(parcel.clone());
        }
    }
    match parcels.len() {
        0 => None,
        _ => Some(parcels),
    }
}

pub fn check_mydxf_in_parcel(mydxf: &MyDxf, parcel: &Parcel) -> bool {
    match mydxf.entities.relate_entities(&parcel.entities) {
        Some(_) => {
            debug!("got intersect with {}", parcel.number);
            true
        },
        None => false,
    }
}


#[cfg(test)]
mod test;