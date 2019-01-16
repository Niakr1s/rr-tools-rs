use dxf;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub r: Option<f64>,
}

impl Point {
    pub fn from_dxf_point(&dxf::Point { x: x, y: y, .. }: &dxf::Point) -> Point {
        Point { x, y, r: None }
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
