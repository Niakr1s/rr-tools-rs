use crate::geometry::entities::*;
use crate::geometry::traits::rectangable::*;
use crate::geometry::traits::relative::*;
use dxf::{Drawing, DxfResult};
use dxf::entities::{self, EntityType};

#[derive(Debug)]
pub struct MyDxf {
    pub path: String,
    pub entities: Entities,
}

impl MyDxf {
    pub fn from_file(path: &str) -> DxfResult<MyDxf> {
        info!("{}: creating MyDxf...", path);
        let path = path.to_string();
        let drawing = Drawing::load_file(&path)?;
        let entities = drawing_to_entities(drawing);
        info!("{}: MyDxf created", path);

        Ok(MyDxf { path, entities })
    }
}

impl Rectangable for MyDxf {
    fn rect(&self) -> Rect {
        self.entities.rect()
    }
}

impl Relative for MyDxf {
    fn relate_entity(&self, entity: &Entity) -> Option<Relation> {
        self.entities.relate_entity(entity)
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
    info!("{:?}", entities);
    entities
}

#[cfg(test)]
mod test;
