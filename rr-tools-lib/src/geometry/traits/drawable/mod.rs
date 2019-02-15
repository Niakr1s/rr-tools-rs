use crate::geometry::entities::*;
use dxf::entities as dxf_entities;
use dxf::entities::Circle as DxfCircle;
use dxf::entities::Vertex;
use dxf::Drawing;
use dxf::Point as DxfPoint;

pub trait Drawable {
    fn push(&self, drawing: &mut Drawing);
    // fn draw(&self, drawing: &mut Drawing) -> DxfResult<()>;
}

impl Drawable for Entities {
    fn push(&self, drawing: &mut Drawing) {
        for e in self {
            e.push(drawing);
        }
    }
}

impl Drawable for Entity {
    fn push(&self, drawing: &mut Drawing) {
        match self {
            Entity::Contur(ref c) => {
                let vertices = c
                    .points
                    .iter()
                    .map(|p| Vertex {
                        location: DxfPoint::new(p.x, p.y, p.get_radius()),
                        ..Default::default()
                    })
                    .collect::<Vec<Vertex>>();
                let dxf_polyline = dxf_entities::Polyline {
                    vertices,
                    ..Default::default()
                };

                drawing.entities.push(dxf_entities::Entity::new(
                    dxf_entities::EntityType::Polyline(dxf_polyline),
                ));
            }
            Entity::Point(ref p) => {
                let center = DxfPoint::new(p.x, p.y, 0.);
                let radius = p.get_radius();
                let dxf_circle = DxfCircle::new(center, radius);
                drawing
                    .entities
                    .push(dxf_entities::Entity::new(dxf_entities::EntityType::Circle(
                        dxf_circle,
                    )));
            }
        };
    }
}

#[cfg(test)]
mod test;
