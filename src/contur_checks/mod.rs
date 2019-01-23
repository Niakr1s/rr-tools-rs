use crate::mydxf::{MyDxf, Entity};
use crate::rrxml::{RrXml, Parcel};

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
//    for entity in &mydxf.entities {
//        match entity {
//            Entity::Contur(contur) -> ,
//            Entity::Point(point) ->
//        }
//    }

    unimplemented!()
}