use crate::mydxf::MyDxf;
use crate::rrxml::RrXml;
use crate::rrxml::Parcel;
use crate::geometry::rect::Rectangable;

pub mod simple_checks;
pub mod entities;
pub mod rect;

/// If true - rects can not intersect physically
/// If false - rects can both intersect or not
fn rects_can_not_intersect(a: &impl Rectangable, b: &impl Rectangable) -> bool {
    let (a, b) = (a.rect(), b.rect());
    a.xmax < b.xmin || a.ymax < b.ymin || a.xmin > b.xmax || a.ymin > b.ymax
}

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