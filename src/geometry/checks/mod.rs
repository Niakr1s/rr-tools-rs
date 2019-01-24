use crate::geometry::entities::*;

pub fn lines_intersect(line1: (&Point, &Point), line2: (&Point, &Point)) -> bool {
    /* Returns True if intersect else False
    line1 = (p1, q1), line2 = (p2, q2),
    where p1,q1,p2,q2 - points like (x,y)->tuple, where x,y - coordinates */

    let ((p1, q1), (p2, q2)) = (line1, line2);

    fn case1(p1: &Point, q1: &Point, p2: &Point, q2: &Point) -> bool {
        /* 1. General Case:
        – (p1, q1, p2) and (p1, q1, q2) have different orientations and
        – (p2, q2, p1) and (p2, q2, q1) have different orientations. */

        let subcase1 = is_on_left_or_right(p1, q1, p2) ^ is_on_left_or_right(p1, q1, q2);

        if subcase1 {
            let subcase2 = is_on_left_or_right(p2, q2, p1) ^ is_on_left_or_right(p2, q2, q1);
            if subcase2 {
                return true;
            }
        }
        return false;
    }

    fn case2(p1: &Point, q1: &Point, p2: &Point, q2: &Point) -> bool {
        /* 2. Special Case
        – (p1, q1, p2), (p1, q1, q2), (p2, q2, p1)
        and (p2, q2, q1) are all collinear and
        – the x-projections of (p1, q1) and (p2, q2) intersect via case1
        – the y-projections of (p1, q1) and (p2, q2) intersect via case1 */

        let subcase1 = is_collinear(p1, q1, p2)
            & is_collinear(p1, q1, q2)
            & is_collinear(p2, q2, p1)
            & is_collinear(p2, q2, q1);

        if subcase1 {
            let (xp1, xq1, xp2, xq2) = (
                x_projection(p1),
                x_projection(q1),
                x_projection(p2),
                x_projection(q2),
            );
            let subcase2 = case1(&xp1, &xq1, &xp2, &xq2);
            if subcase2 {
                let (yp1, yq1, yp2, yq2) = (
                    y_projection(p1),
                    y_projection(q1),
                    y_projection(p2),
                    y_projection(q2),
                );
                let subcase3 = case1(&yp1, &yq1, &yp2, &yq2);
                if subcase3 {
                    return true;
                }
            }
        }
        return false;
    }

    fn area(a: &Point, b: &Point, c: &Point) -> f64 {
        (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)
    }

    fn is_on_left_or_right(a: &Point, b: &Point, c: &Point) -> bool {
        area(a, b, c) > 0.
    }

    fn is_collinear(a: &Point, b: &Point, c: &Point) -> bool {
        area(a, b, c) == 0.
    }

    fn x_projection(p: &Point) -> Point {
        Point {
            x: p.x,
            y: 0.,
            r: p.r,
        }
    }

    fn y_projection(p: &Point) -> Point {
        Point {
            x: 0.,
            y: p.y,
            r: p.r,
        }
    }

    if case1(p1, q1, p2, q2) {
        return true;
    } else if case2(p1, q1, p2, q2) {
        return true;
    };
    false
}

pub fn point_inside_contur(p: &Point, c: &Contur) -> bool {
    if !c.is_closed() {
        return false;
    };
    let n = c.points.len();
    let mut inside = false;
    let mut p1 = &c.points[0];
    for i in 1..=n {
        let p2 = &c.points[i % n];
        if p.y > p1.y.min(p2.y) {
            if p.y <= p1.y.max(p2.y) {
                if p.x <= p1.x.max(p2.x) {
                    if p1.y != p2.y {
                        let xinters = (p.y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y) + p1.x;
                        if p.x <= xinters {
                            inside = !inside;
                        }
                    };
                    if p1.x == p2.x {
                        inside = !inside;
                    };
                }
            }
        }
        p1 = &p2;
    }

    inside
}

pub fn circle_intersect_line(circle: &Point, line: (&Point, &Point)) -> bool {
    /* algorithm:
    http://pers.narod.ru/algorithms/pas_dist_from_point_to_line.html */

    let &Point { x: x0, y: y0, r: radius } = circle;
    let &Point { x: x1, y: y1, .. } = line.0;
    let &Point { x: x2, y: y2, .. } = line.1;

    fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).powf(0.5)
    }

    let radius = match radius {
        Some(r) => r,
        None => 0.,
    };

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
    unimplemented!()
}

#[cfg(test)]
mod test;
