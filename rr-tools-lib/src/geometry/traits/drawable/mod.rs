use crate::geometry::entities::*;
use crate::mydxf::MyDxf;
use crate::rrxml::RrXml;
use dxf::entities as dxf_entities;
use dxf::entities::Circle as DxfCircle;
use dxf::entities::Vertex;
use dxf::Drawing;
use dxf::Point as DxfPoint;

pub trait Drawable {
    fn draw(&self, drawing: &mut Drawing);
    // fn draw(&self, drawing: &mut Drawing) -> DxfResult<()>;
}

impl Drawable for MyDxf {
    fn draw(&self, drawing: &mut Drawing) {
        self.entities.draw(drawing);
    }
}

impl Drawable for RrXml {
    fn draw(&self, drawing: &mut Drawing) {}
}

impl Drawable for Entities {
    fn draw(&self, drawing: &mut Drawing) {
        for e in self {
            e.draw(drawing);
        }
    }
}

impl Drawable for Entity {
    fn draw(&self, drawing: &mut Drawing) {
        match self {
            Entity::Contur(ref c) => {
                let vertices = c
                    .points
                    .iter()
                    .map(|p| Vertex {
                        location: DxfPoint::new(p.y, p.x, p.get_radius()),
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
                let center = DxfPoint::new(p.y, p.x, 0.);
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
