use crate::geometry::rect::*;
use dxf::entities::Circle as DxfCircle;
use dxf::Point as DxfPoint;

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub r: Option<f64>,
}

impl Point {
    pub fn new(x: f64, y: f64, r: Option<f64>) -> Point {
        Point { x, y, r }
    }

    // should be reversed
    pub fn from_dxf_point(DxfPoint { x, y, .. }: &DxfPoint) -> Point {
        Point {
            x: *y,
            y: *x,
            r: None,
        }
    }

    pub fn from_dxf_circle(DxfCircle { center, radius, .. }: &DxfCircle) -> Point {
        let p = Point::from_dxf_point(&center);
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

impl Rectangable for Point {
    fn rect(&self) -> Rect {
        let mut rect = Rect::new();
        rect.add_point(self);
        rect
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Contur {
    pub points: Vec<Point>,
}

impl Contur {
    pub fn new() -> Contur {
        Contur { points: vec![] }
    }
    pub fn add(&mut self, p: Point) {
        self.points.push(p)
    }
    pub fn is_closed(&self) -> bool {
        match self.points.last() {
            Some(l) => {
                if self.points[0] != *l {
                    return false;
                }
            }
            None => return false,
        };
        true
    }
    pub fn len(&self) -> usize {
        self.points.len()
    }
}

impl Rectangable for Contur {
    fn rect(&self) -> Rect {
        let mut rect = Rect::new();
        for p in &self.points {
            rect.add_point(p);
        };
        rect
    }
}