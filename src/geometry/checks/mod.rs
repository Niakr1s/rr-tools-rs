use crate::geometry::entities::*;

/// see https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
/// for logics explanation
pub fn lines_intersect(line1: (&Point, &Point), line2: (&Point, &Point)) -> bool {
    let ((p1, q1), (p2, q2)) = (line1, line2);

    fn on_segment(p: &Point, q: &Point, r: &Point) -> bool {
        q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) &&
            q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
    }

    fn orientation(p: &Point, q: &Point, r: &Point) -> i32 {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        if val == 0. { return 0 };
        match val > 0. {
            true => 1,
            false => 2,
        }
    }

    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    if o1 != o2 && o3 != o4 { return true };
    if o1 == 0 && on_segment(p1, p2, q1) { return true };
    if o2 == 0 && on_segment(p1, q2, q1) { return true };
    if o3 == 0 && on_segment(p2, p1, q2) { return true };
    if o4 == 0 && on_segment(p2, q1, q2) { return true };
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
                    let xinters = if p1.y != p2.y {
                        Some((p.y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y) + p1.x)
                    } else { None };
                    let x_le_xinters = match xinters {
                        Some(xinters) => p.x <= xinters,
                        None => false,
                    };
                    if p1.x == p2.x || x_le_xinters {
                        inside = !inside;
                    };
                };
            }
        }
        p1 = p2;
    }
    inside
}

pub fn circle_intersect_line(circle: &Point, line: (&Point, &Point)) -> bool {
    /* algorithm:
    http://pers.narod.ru/algorithms/pas_dist_from_point_to_line.html */

    let &Point { x: x0, y: y0, r: radius } = circle;
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
    let &Point { x: x1, y: y1, r: r1 } = c1;
    let &Point { x: x2, y: y2, r: r2 } = c2;
    let distance = dist(x1, y1, x2, y2);
    let max_distance = r1.unwrap_or(0.) + r2.unwrap_or(0.);
    distance <= max_distance
}

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).powf(0.5)
}

#[cfg(test)]
mod test;
