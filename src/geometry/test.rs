use crate::geometry::entities::Contur;
use crate::geometry::entities::Point;
use super::*;

#[test]
fn rects_can_not_intersect_ok() {
    let p = Point::new(1., 2., Some(3.));  // got Rect {4 5 -2 -1}

    let mut c = Contur::new();
    c.add(Point::new(2., 3., None));
    c.add(Point::new(-2., -3., None));
    c.add(Point::new(4., 2., None));
    // got Rect { 4 3 -2 -3 }

    assert!(!p.can_not_intersect(&c));

    let p = Point::new(5., 0., None);  // got Rect {5 0 5 0}

    assert!(p.can_not_intersect(&c));
}