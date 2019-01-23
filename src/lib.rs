#[macro_use]
extern crate log;

pub mod geometry;
pub mod mydxf;
pub mod rrxml;
pub mod error;
pub mod contur_checks;

use crate::geometry::entities::Rect;

trait Rectangable {
    fn rect(&self) -> Rect;
}