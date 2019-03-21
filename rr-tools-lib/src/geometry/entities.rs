use dxf::entities as dxf_entities;
use dxf::entities::Circle as DxfCircle;
use dxf::entities::Entity as DxfEntity;
use dxf::entities::Vertex;
use dxf::Color as DxfColor;
use dxf::Point as DxfPoint;

pub type Entities = Vec<Entity>;

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

    pub fn to_dxf_entity(&self, color: DxfColor) -> DxfEntity {
        let mut dxf_entity = match self {
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

                dxf_entities::Entity::new(dxf_entities::EntityType::Polyline(dxf_polyline))
            }
            Entity::Point(ref p) => {
                let center = DxfPoint::new(p.y, p.x, 0.);
                let radius = p.get_radius();
                let dxf_circle = DxfCircle::new(center, radius);

                dxf_entities::Entity::new(dxf_entities::EntityType::Circle(dxf_circle))
            }
        };
        dxf_entity.common.color = color;
        dxf_entity
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

    pub fn get_radius(&self) -> f64 {
        match self.r {
            Some(r) => r,
            None => 0.,
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

#[macro_export]
macro_rules! contur {
    ( $( $x:expr ),* ) => {
        {
            let mut temp = Contur::new();
            $(
                temp.push($x);
            )*
            temp
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::geometry::traits::rectangable::Rectangable;

    #[test]
    fn eq_points_ref() {
        assert_eq!(
            &Point::new(1.0, 1.0, Some(1.0)),
            &Point::new(1.0, 1.0, Some(1.0))
        )
    }

    #[test]
    fn rects_can_not_intersect_ok() {
        let p = Point::new(1., 2., Some(3.)); // got Rect {4 5 -2 -1}

        let c = contur![
            Point::new(2., 3., None),
            Point::new(-2., -3., None),
            Point::new(4., 2., None)
        ];
        // got Rect { 4 3 -2 -3 }

        assert!(!p.can_not_intersect(&c));

        let p = Point::new(5., 0., None); // got Rect {5 0 5 0}

        assert!(p.can_not_intersect(&c));
    }
}
