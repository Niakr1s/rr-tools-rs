use crate::geometry::rect::Rectangable;
use crate::mydxf::MyDxf;
use crate::rrxml::Parcel;
use crate::rrxml::RrXml;

pub mod simple_checks;
pub mod entities;
pub mod rect;

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
    // todo
//    for entity in &mydxf.entities {
//        match entity {
//            Entity::Contur(contur) -> ,
//            Entity::Point(point) ->
//        }
//    }

    unimplemented!()
}

#[cfg(test)]
mod test;