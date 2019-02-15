use crate::geometry::entities::*;
use crate::mydxf::MyDxf;
use crate::rrxml::RrXml;
use dxf::entities as dxf_entities;
use dxf::entities::Circle as DxfCircle;
use dxf::entities::Vertex;
use dxf::Point as DxfPoint;
use dxf::{Color, Drawing};

pub trait DrawableWithColor {
    fn draw(&self, drawing: &mut Drawing, color: u8);
    // fn draw(&self, drawing: &mut Drawing) -> DxfResult<()>;
}

impl DrawableWithColor for Entities {
    fn draw(&self, drawing: &mut Drawing, color: u8) {
        for e in self {
            e.draw(drawing, color);
        }
    }
}

impl DrawableWithColor for Entity {
    fn draw(&self, drawing: &mut Drawing, color: u8) {
        let color = Color::from_index(color);
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

                let mut entity =
                    dxf_entities::Entity::new(dxf_entities::EntityType::Polyline(dxf_polyline));

                entity.common.color = color;

                drawing.entities.push(entity);
            }
            Entity::Point(ref p) => {
                let center = DxfPoint::new(p.y, p.x, 0.);
                let radius = p.get_radius();
                let dxf_circle = DxfCircle::new(center, radius);

                let mut entity =
                    dxf_entities::Entity::new(dxf_entities::EntityType::Circle(dxf_circle));

                entity.common.color = color;

                drawing.entities.push(entity);
            }
        };
    }
}

#[cfg(test)]
mod test;
