use crate::geometry::entities::*;
use dxf::entities as dxf_entities;
use dxf::entities::Circle as DxfCircle;
use dxf::Point as DxfPoint;
use dxf::{Drawing, DxfResult, LwPolylineVertex};

pub trait Drawable {
    fn draw(&self, drawing: &mut Drawing) -> DxfResult<()>;
}

impl Drawable for Entity {
    fn draw(&self, drawing: &mut Drawing) -> DxfResult<()> {
        match self {
            Entity::Contur(ref c) => {
                let vertices = c
                    .points
                    .iter()
                    .map(|point| LwPolylineVertex {
                        x: point.x,
                        y: point.y,
                        ..Default::default()
                    })
                    .collect::<Vec<LwPolylineVertex>>();
                let dxf_polyline = dxf_entities::LwPolyline {
                    vertices,
                    ..Default::default()
                };

                drawing.entities.push(dxf_entities::Entity::new(
                    dxf_entities::EntityType::LwPolyline(dxf_polyline),
                ));
            }
            Entity::Point(ref p) => {
                println!("{}", p.x);
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
        Ok(())
    }
}
