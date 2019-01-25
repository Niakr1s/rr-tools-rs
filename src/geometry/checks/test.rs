use super::*;

#[test]
fn lines_intersect_check1() {
    // should be false
    let seg1 = (
        &Point::new(1., 1., None),
        &Point::new(10., 1., None),
    );
    let seg2 = (
        &Point::new(1., 2., None),
        &Point::new(10., 2., None),
    );
    assert!(!lines_intersect(seg1, seg2));
}

#[test]
fn lines_intersect_check2() {
    // should be true
    let seg1 = (
        &Point::new(10., 0., None),
        &Point::new(0., 10., None),
    );
    let seg2 = (
        &Point::new(0., 0., None),
        &Point::new(10., 10., None),
    );
    assert!(lines_intersect(seg1, seg2));
}
#[test]
fn lines_intersect_check3() {
    // should be false
    let seg1 = (
        &Point::new(-5., -5., None),
        &Point::new(0., 0., None),
    );
    let seg2 = (
        &Point::new(1., 1., None),
        &Point::new(10., 10., None),
    );
    assert!(!lines_intersect(seg1, seg2));
}

#[test]
fn same_lines_intersect() {
    let seg1 = (
        &Point::new(10., 0., None),
        &Point::new(0., 0., None),
    );
    assert!(lines_intersect(seg1, seg1.clone()));
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
            Point::new(-3., -3., None)
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
            Point::new(-3., -3., None)
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
            Point::new(-3., -3., None)
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
    let poly1 = (
        Point::new(2., -1., None),
        Point::new(2., 1., None),
    );
    let poly2 = (
        Point::new(2., -1., None),
        Point::new(2., 0., None),
    );
    let poly3 = (
        Point::new(2., -1., None),
        Point::new(2., -0.001, None),
    );
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
    let poly = (
        Point::new(1., 1., None),
        Point::new(0., 0., None),
    );
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