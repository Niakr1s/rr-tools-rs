use crate::geometry::entities::*;
// use dxf::entities::*;
use dxf::entities::{self, EntityType};
use dxf::{Drawing, DxfResult};

enum Entity {
    Contur(Contur),
    Line(Line),
    Point(Point),
}

pub struct MyDxf {
    path: String,
    drawing: Drawing,
    entities: Vec<Entity>,
}

impl MyDxf {
    pub fn from_file(path: &str) -> DxfResult<MyDxf> {
        let drawing = Drawing::load_file(path)?;
        let path = path.to_string();
        let entities = vec![];
        Ok(MyDxf {
            path,
            drawing,
            entities,
        })
    }
}

fn drawing_to_entities(drawing: &Drawing) -> Vec<Entity> {
    let mut entities = vec![];
    // for e in drawing.entities.as_ref() {
    //     // match e.specific {
    //     //     EntityType::LwPolyline(lw_polyline) => {
    //     //         for
    //     //     }
    //     //     // entities::Polyline => {}
    //     //     // entities::Line => {}
    //     //     // entities::Circle => {}
    //     //     // entitites::ModelPoint => {}
    //     //     _ => (),
    //     // }
    // }
    entities
}

#[cfg(test)]
mod test;
