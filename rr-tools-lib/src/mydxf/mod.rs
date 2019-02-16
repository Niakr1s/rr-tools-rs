use crate::geometry::entities::*;
use crate::geometry::traits::rectangable::*;
use dxf::entities::EntityType;
use dxf::{Drawing, DxfResult};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct MyDxf {
    pub path: String,
    pub entities: Entities,
}

impl MyDxf {
    pub fn from_file(path: &str) -> DxfResult<MyDxf> {
        debug!("attempt to parse dxf: {}", path);
        let path = path.to_string();
        let drawing = Drawing::load_file(&path)?;
        let entities = drawing_to_entities(drawing);
        let parsed = MyDxf { path, entities };
        debug!("succesfully parsed dxf: {}", parsed);

        Ok(parsed)
    }
}

impl Display for MyDxf {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "MyDxf from {path}", path = self.path,)?;
        writeln!(f, "with {} entities", self.entities.len())?;
        writeln!(f)
    }
}

impl Rectangable for MyDxf {
    fn rect(&self) -> Rect {
        self.entities.rect()
    }
}

fn drawing_to_entities(drawing: Drawing) -> Vec<Entity> {
    let mut entities = vec![];
    for e in drawing.entities {
        let contur = match e.specific {
            EntityType::LwPolyline(lw_polyline) => {
                let mut contur = Contur::new();
                for p in lw_polyline.vertices {
                    contur.push(Point::new(p.y, p.x, None)); // should be reversed
                }
                Entity::Contur(contur)
            }
            EntityType::Polyline(polyline) => {
                let mut contur = Contur::new();
                for p in polyline.vertices {
                    contur.push(Point::from_dxf_point(&p.location));
                }
                Entity::Contur(contur)
            }
            EntityType::Line(line) => {
                let p1 = Point::from_dxf_point(&line.p1);
                let p2 = Point::from_dxf_point(&line.p2);
                Entity::Contur(contur![p1, p2])
            }
            EntityType::Circle(circle) => Entity::Point(Point::from_dxf_circle(&circle)),
            EntityType::ModelPoint(model_point) => {
                Entity::Point(Point::from_dxf_point(&model_point.location))
            }
            _ => continue,
        };
        entities.push(contur);
    }
    entities
}

#[cfg(test)]
mod test;
