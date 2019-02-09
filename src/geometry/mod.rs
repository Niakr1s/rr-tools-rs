use crate::geometry::entities::*;
use crate::geometry::traits::rectangable::Rectangable;
use crate::geometry::traits::relative::Relative;
use crate::mydxf::MyDxf;
use crate::rrxml::Parcel;
use crate::rrxml::RrXml;

pub mod checks;
#[macro_use]
pub mod entities;
pub mod traits;

fn check_mydxf_in_rrxml(mydxf: &MyDxf, rrxml: &RrXml) -> Option<Vec<Parcel>> {
    if rrxml.len() == 0 { return None };

    let mut parcels = vec![];

    for parcel in &rrxml.parcels {
        if check_mydxf_in_parcel(&mydxf, &parcel) {
            parcels.push(parcel.clone());
        }
    }

    Some(parcels)
}

fn check_mydxf_in_parcel(mydxf: &MyDxf, parcel: &Parcel) -> bool {
    match mydxf.relate_entities(&parcel.entities) {
        Some(_) => true,
        None => false,
    }
}

#[cfg(test)]
mod test;