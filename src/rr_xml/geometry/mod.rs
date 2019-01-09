use crate::rr_xml::Point;

fn is_intersect(segment1: &(Point, Point), segment2: &(Point, Point)) -> bool {
    /* Returns True if intersect else False
    segment1 = (p1, q1), segment2 = (p2, q2),
    where p1,q1,p2,q2 - points like (x,y)->tuple, where x,y - coordinates */

    let ((p1, q1), (p2, q2)) = (segment1, segment2);

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

#[cfg(test)]
mod test;
