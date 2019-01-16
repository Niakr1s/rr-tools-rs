use dxf::entities::Circle as DxfCircle;
use dxf::Point as DxfPoint;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub r: Option<f64>,
}

impl Point {
    pub fn from_dxf_point(&DxfPoint { x, y, .. }: &DxfPoint) -> Point {
        Point { x, y, r: None }
    }
    pub fn from_dxf_circle(
        DxfCircle {
            center: dxf_point,
            radius,
            ..
        }: &DxfCircle,
    ) -> Point {
        let p = Point::from_dxf_point(&dxf_point);
        Point {
            r: Some(*radius),
            ..p
        }
    }
    pub fn is_circle(&self) -> bool {
        match self.r {
            Some(_) => true,
            None => false,
        }
    }
    pub fn is_point(&self) -> bool {
        match self.r {
            Some(_) => false,
            None => true,
        }
    }
}
