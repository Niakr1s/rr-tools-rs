use crate::geometry::entities::*;

/// see https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
/// for logics explanation
pub fn lines_intersect(line1: (&Point, &Point), line2: (&Point, &Point)) -> bool {
    let ((p1, q1), (p2, q2)) = (line1, line2);

    /// Given three colinear points p, q, r, the function checks if
    /// point q lies on line segment 'pr'
    fn on_segment(p: &Point, q: &Point, r: &Point) -> bool {
        q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) && q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
    }

    /// To find orientation of ordered triplet (p, q, r).
    /// The function returns following values
    /// 0 --> p, q and r are colinear
    /// 1 --> Clockwise
    /// 2 --> Counterclockwise
    fn orientation(p: &Point, q: &Point, r: &Point) -> i32 {
        // See https://www.geeksforgeeks.org/orientation-3-ordered-points/
        // for details of below formula.
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        if val == 0. {
            return 0;
        }; //collinear
        match val > 0. {
            true => 1,  //clockwise
            false => 2, //counterclockwise
        }
    }

    // Find the four orientations needed for general and
    // special cases
    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    };
    // Special Cases
    // p1, q1 and p2 are colinear and p2 lies on segment p1q1
    if o1 == 0 && on_segment(p1, p2, q1) {
        return true;
    };
    // p1, q1 and q2 are colinear and q2 lies on segment p1q1
    if o2 == 0 && on_segment(p1, q2, q1) {
        return true;
    };
    // p2, q2 and p1 are colinear and p1 lies on segment p2q2
    if o3 == 0 && on_segment(p2, p1, q2) {
        return true;
    };
    // p2, q2 and q1 are colinear and q1 lies on segment p2q2
    if o4 == 0 && on_segment(p2, q1, q2) {
        return true;
    };
    false
}

/// both for circles and points
pub fn circle_inside_contur(p: &Point, c: &Contur) -> bool {
    let inside = if c.is_closed() {
        let mut inside = false;
        // here inside can change
        let n = c.points.len();
        let mut p1 = &c.points[0];
        for i in 1..=n {
            let p2 = &c.points[i % n];
            if p.y > p1.y.min(p2.y) && p.y <= p1.y.max(p2.y) && p.x <= p1.x.max(p2.x) {
                let xinters = if (p1.y - p2.y).abs() > 0.000_001 {
                    Some((p.y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y) + p1.x)
                } else {
                    None
                };
                let x_le_xinters = match xinters {
                    Some(xinters) => p.x <= xinters,
                    None => false,
                };
                if (p1.x - p2.x).abs() < 0.000_001 || x_le_xinters {
                    inside = !inside;
                };
            }
            p1 = p2;
        }
        inside
    } else {
        false
    };

    if inside && p.is_circle() {
        return !circle_relate_contur(p, c);
    };
    inside
}

pub fn circle_relate_contur(p: &Point, c: &Contur) -> bool {
    let other_points = &c.points;

    let mut other_first = other_points.first().unwrap();
    for other_p in other_points {
        if circle_relate_line(p, (other_first, other_p)) {
            return true;
        };
        other_first = other_p;
    }
    false
}

/// algorithm:
/// http://pers.narod.ru/algorithms/pas_dist_from_point_to_line.html
pub fn circle_relate_line(circle: &Point, line: (&Point, &Point)) -> bool {
    let &Point {
        x: x0,
        y: y0,
        r: radius,
    } = circle;
    let &Point { x: x1, y: y1, .. } = line.0;
    let &Point { x: x2, y: y2, .. } = line.1;

    let radius = radius.unwrap_or(0.);

    let r1 = dist(x0, y0, x1, y1);
    let r2 = dist(x0, y0, x2, y2);
    let r12 = dist(x1, y1, x2, y2);
    let res = if r1 >= dist(r2, r12, 0., 0.) {
        r2
    } else if r2 >= dist(r1, r12, 0., 0.) {
        r1
    } else {
        let mut a = y2 - y1;
        let mut b = x1 - x2;
        let mut c = -x1 * (y2 - y1) + y1 * (x2 - x1);
        let t = dist(a, b, 0., 0.);
        if c > 0. {
            a = -a;
            b = -b;
            c = -c;
        }
        (a * x0 + b * y0 + c) / t
    }
    .abs();
    res <= radius
}

pub fn circle_intersect_circle(c1: &Point, c2: &Point) -> bool {
    let &Point {
        x: x1,
        y: y1,
        r: r1,
    } = c1;
    let &Point {
        x: x2,
        y: y2,
        r: r2,
    } = c2;
    let distance = dist(x1, y1, x2, y2);
    let max_distance = r1.unwrap_or(0.) + r2.unwrap_or(0.);
    distance <= max_distance
}

/// checks c1 inside c2, not vice versa
pub fn circle_inside_circle(c1: &Point, c2: &Point) -> bool {
    let dist = dist(c1.x, c1.y, c2.x, c2.y);
    let r1 = c1.r.unwrap_or(0.);
    let r2 = c2.r.unwrap_or(0.);
    dist + r1 < r2
}

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).powf(0.5)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lines_intersect_check1() {
        // should be false
        let seg1 = (&Point::new(1., 1., None), &Point::new(10., 1., None));
        let seg2 = (&Point::new(1., 2., None), &Point::new(10., 2., None));
        assert!(!lines_intersect(seg1, seg2));
    }

    #[test]
    fn lines_intersect_check2() {
        // should be true
        let seg1 = (&Point::new(10., 0., None), &Point::new(0., 10., None));
        let seg2 = (&Point::new(0., 0., None), &Point::new(10., 10., None));
        assert!(lines_intersect(seg1, seg2));
    }
    #[test]
    fn lines_intersect_check3() {
        // should be false
        let seg1 = (&Point::new(-5., -5., None), &Point::new(0., 0., None));
        let seg2 = (&Point::new(1., 1., None), &Point::new(10., 10., None));
        assert!(!lines_intersect(seg1, seg2));
    }

    #[test]
    fn same_lines_intersect() {
        let seg1 = (&Point::new(10., 0., None), &Point::new(0., 0., None));
        assert!(lines_intersect(seg1, seg1));
    }

    #[test]
    fn point_inside_contur_check1_ok() {
        let p = Point::new(1., 1., None);
        let c = Contur {
            points: vec![
                Point::new(-2., -2., None),
                Point::new(2., -2., None),
                Point::new(1., 2., None),
                Point::new(-2., -2., None),
            ],
        };
        assert!(circle_inside_contur(&p, &c));
        let c = Contur {
            points: vec![
                Point::new(-3., -3., None),
                Point::new(-3., 3., None),
                Point::new(3., 3., None),
                Point::new(3., -3., None),
                Point::new(-3., -3., None),
            ],
        };
        assert!(circle_inside_contur(&p, &c));
    }

    #[test]
    fn point_inside_contur_check2_err() {
        let p = Point::new(1., 1., None);
        let c = Contur {
            points: vec![
                Point::new(-2., -2., None),
                Point::new(2., -2., None),
                Point::new(2., 0., None),
                Point::new(-2., -2., None),
            ],
        };
        assert!(!circle_inside_contur(&p, &c));
    }

    #[test]
    fn circle_inside_contur_ok() {
        let p = Point::new(0., 0., Some(2.99));
        let c = Contur {
            points: vec![
                Point::new(-3., -3., None),
                Point::new(-3., 3., None),
                Point::new(3., 3., None),
                Point::new(3., -3., None),
                Point::new(-3., -3., None),
            ],
        };
        assert!(circle_inside_contur(&p, &c));

        let p = Point::new(0., 0., Some(3.01));
        assert!(!circle_inside_contur(&p, &c));
    }

    #[test]
    fn circle_relate_contur_ok() {
        let p = Point::new(0., 0., Some(2.99));
        let c = Contur {
            points: vec![
                Point::new(-3., -3., None),
                Point::new(-3., 3., None),
                Point::new(3., 3., None),
                Point::new(3., -3., None),
                Point::new(-3., -3., None),
            ],
        };
        assert!(!circle_relate_contur(&p, &c));

        let p = Point::new(0., 0., Some(3.01));
        assert!(circle_relate_contur(&p, &c));
    }

    #[test]
    fn circle_relate_line_check() {
        let c1 = Point::new(0., 0., Some(1.));
        let c2 = Point::new(0., 0., Some(2.));
        let c3 = Point::new(0., 0., Some(3.));
        let p = Point::new(2., 0., None);
        let poly1 = (Point::new(2., -1., None), Point::new(2., 1., None));
        let poly2 = (Point::new(2., -1., None), Point::new(2., 0., None));
        let poly3 = (Point::new(2., -1., None), Point::new(2., -0.001, None));
        assert!(!circle_relate_line(&c1, (&poly1.0, &poly1.1)));
        assert!(circle_relate_line(&c2, (&poly1.0, &poly1.1)));
        assert!(circle_relate_line(&c3, (&poly1.0, &poly1.1)));
        assert!(circle_relate_line(&p, (&poly1.0, &poly1.1)));

        assert!(!circle_relate_line(&c1, (&poly2.0, &poly2.1)));
        assert!(circle_relate_line(&c2, (&poly2.0, &poly2.1)));
        assert!(circle_relate_line(&c3, (&poly2.0, &poly2.1)));
        assert!(circle_relate_line(&p, (&poly2.0, &poly2.1)));

        assert!(!circle_relate_line(&c1, (&poly3.0, &poly3.1)));
        assert!(!circle_relate_line(&c2, (&poly3.0, &poly3.1)));
        assert!(circle_relate_line(&c3, (&poly3.0, &poly3.1)));
        assert!(!circle_relate_line(&p, (&poly3.0, &poly3.1)));
    }

    #[test]
    fn circle_relate_line_inside() {
        let c = Point::new(0., 0., Some(3.));
        let poly = (Point::new(1., 1., None), Point::new(0., 0., None));
        assert!(circle_relate_line(&c, (&poly.0, &poly.1)));
    }

    #[test]
    fn circle_intersect_circle_check() {
        let c1 = Point::new(0., 0., Some(2.));
        let c2 = Point::new(4., 0., Some(2.));
        assert!(circle_intersect_circle(&c1, &c2));
        let c2 = Point::new(4., 0.01, Some(2.));
        assert!(!circle_intersect_circle(&c1, &c2));
        let c2 = Point::new(2., 0., None);
        assert!(circle_intersect_circle(&c1, &c2));
        let c2 = Point::new(0., 0., Some(1.));
        assert!(circle_intersect_circle(&c1, &c2));
    }

    #[test]
    fn circle_inside_circle_check() {
        let c1 = Point::new(1., 0., Some(0.99));
        let c2 = Point::new(0., 0., Some(2.));
        assert!(circle_inside_circle(&c1, &c2));
        let c1 = Point::new(1., 0., Some(1.));
        assert!(!circle_inside_circle(&c1, &c2));
        let c1 = Point::new(1., 0., None);
        assert!(circle_inside_circle(&c1, &c2));
        let c1 = Point::new(4., 0., Some(1.));
        assert!(!circle_inside_circle(&c1, &c2));
    }

}
