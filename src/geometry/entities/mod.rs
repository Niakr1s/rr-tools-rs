use crate::geometry::checks::*;
use crate::geometry::traits::rectangable::*;
use crate::geometry::traits::relative::*;
use dxf::entities::Circle as DxfCircle;
use dxf::Point as DxfPoint;

pub type Entities = Vec<Entity>;

impl Rectangable for Entities {
    fn rect(&self) -> Rect {
        let mut rect = Rect::new();
        for e in self {
            rect.add(e);
        };
        rect
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Entity {
    Contur(Contur),
    Point(Point),
}

impl Entity {
    pub fn from_point(p: Point) -> Entity {
        Entity::Point(p)
    }

    /// Can return Some(Entity::Contur), Some(Entity::Point) and None
    pub fn from_contur(c: Contur) -> Option<Entity> {
        let mut c = c;
        match c.len() {
            0 => None,
            1 => Some(Entity::Point(c.points.pop().unwrap())),
            _ => Some(Entity::Contur(c)),
        }
    }
}

impl Rectangable for Entity {
    fn rect(&self) -> Rect {
        let mut rect = Rect::new();
        match self {
            Entity::Contur(ref c) => rect.add(c),
            Entity::Point(ref p) => rect.add(p),
        }
        rect
    }
}

impl Relative for Entity {
    fn relate_entity(&self, entity: &Entity) -> bool {
        if self.can_not_intersect(entity) { return false };

        match self {
            // Point
            Entity::Point(ref self_point) => match entity {
                Entity::Point(ref other_point) => {
                    // simple circles check
                    circle_intersect_circle(self_point, other_point)
                },
                Entity::Contur(ref other_contur) => {
                    if point_inside_contur(self_point, other_contur) { return true };

                    let other_points = &other_contur.points;
                    let mut other_iter = other_points.iter();
                    let mut other_first = other_iter.next().unwrap();

                    for other_p in other_iter {
                        if circle_intersect_line(self_point, (other_first, other_p)) { return true };
                        other_first = other_p;
                    };

                    false
                },
            },
            // Contur
            Entity::Contur(ref self_contur) => {
                // flags for checking other polygon in self and vice versa
                let mut other_inpolygon = self_contur.is_closed();  // possibly true
                let mut self_inpolygon = None;  // None if entity is Entity::Point

                let self_points = &self_contur.points;
                let mut self_iter = self_points.iter();
                let mut self_first = self_iter.next().unwrap();

                for self_p in self_iter {
                    match entity {
                        Entity::Point(ref other_point) => {
                            if other_inpolygon { other_inpolygon = point_inside_contur(other_point, self_contur) };
                            if circle_intersect_line(other_point, (self_first, self_p)) { return true };
                        },
                        Entity::Contur(ref other_contur) => {
                            let mut self_inpolygon_inner = other_contur.is_closed();

                            let other_points = &other_contur.points;
                            let mut other_iter = other_points.iter();
                            let mut other_first = other_iter.next().unwrap();

                            for other_p in other_iter {
                                let self_segment = (self_first, self_p);
                                let other_segment = (other_first, other_p);
                                // immediatly return if lines are intersecting each other
                                if lines_intersect(self_segment, other_segment) { return true };

                                // updating inpolygon flags
                                if self_inpolygon_inner { self_inpolygon_inner = point_inside_contur(self_first, other_contur) };
                                if other_inpolygon { other_inpolygon = point_inside_contur(other_first, self_contur) };

                                other_first = other_p;
                            };

                            // updating outer flag
                            self_inpolygon = match self_inpolygon {
                                Some(b) => Some(b && self_inpolygon_inner),
                                None => Some(self_inpolygon_inner),
                            };
                        },
                    }
                    self_first = self_p;
                };

                match self_inpolygon {
                    Some(b) => b || other_inpolygon,
                    None => other_inpolygon,
                }
            },
        }
    }
}

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
        match self.r {
            Some(r) => Rect::from(self.x + r, self.y + r, self.x - r, self.y - r).unwrap(),
            None => Rect::from(self.x, self.y, self.x, self.y).unwrap(),
        }
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
    pub fn push(&mut self, p: Point) {
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
            rect.add(p);
        };
        rect
    }
}

#[cfg(test)]
mod test;