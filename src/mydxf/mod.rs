use crate::geometry::entities::{Line, Point};
use dxf::entities::*;
use dxf::{Drawing, DxfResult};

pub struct MyDxf {
    path: String,
    drawing: Drawing,
    lines: Vec<Line>,
    points: Vec<Point>,
}

impl MyDxf {
    pub fn from_file(path: &str) -> DxfResult<MyDxf> {
        let drawing = Drawing::load_file(path)?;
        let path = path.to_string();
        let lines = vec![];
        let points = vec![];
        Ok(MyDxf {
            path,
            drawing,
            lines,
            points,
        })
    }
}

#[cfg(test)]
mod test;
