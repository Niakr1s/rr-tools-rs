use crate::geometry::entities::*;
use crate::geometry::traits::drawable_with_color::DrawableWithColor;
use crate::mydxf::MyDxf;
use crate::rrxml::RrXml;
use dxf::entities as dxf_entities;
use dxf::entities::Circle as DxfCircle;
use dxf::entities::Vertex;
use dxf::Point as DxfPoint;
use dxf::{Color, Drawing};

pub trait Drawable: DrawableWithColor {
    fn draw(&self, drawing: &mut Drawing) {
        self.draw_with_color(drawing, 7);
    }
}

#[cfg(test)]
mod test;
