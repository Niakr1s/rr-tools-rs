use super::*;

#[test]
fn is_intersect_check1() {
    // should be false
    let seg1 = (
        &Point::new(1., 1., None),
        &Point::new(10., 1., None),
    );
    let seg2 = (
        &Point::new(1., 2., None),
        &Point::new(10., 2., None),
    );
    assert!(!is_intersect(seg1, seg2));
}

#[test]
fn is_intersect_check2() {
    // should be true
    let seg1 = (
        &Point::new(10., 0., None),
        &Point::new(0., 10., None),
    );
    let seg2 = (
        &Point::new(0., 0., None),
        &Point::new(10., 10., None),
    );
    assert!(is_intersect(seg1, seg2));
}
#[test]
fn is_intersect_check3() {
    // should be false
    let seg1 = (
        &Point::new(-5., -5., None),
        &Point::new(0., 0., None),
    );
    let seg2 = (
        &Point::new(1., 1., None),
        &Point::new(10., 10., None),
    );
    assert!(!is_intersect(seg1, seg2));
}

#[test]
fn inside_polygon_check1() {
    let p = Point::new(1., 1., None);
    let c = Contur {
        points: vec![
            Point::new(-2., -2., None),
            Point::new(2., -2., None),
            Point::new(1., 2., None),
            Point::new(-2., -2., None),
        ],
    };
    assert!(inside_polygon(&p, &c));
}

#[test]
fn inside_polygon_check2() {
    let p = Point::new(1., 1., None);
    let c = Contur {
        points: vec![
            Point::new(-2., -2., None),
            Point::new(2., -2., None),
            Point::new(2., 0., None),
            Point::new(-2., -2., None),
        ],
    };
    assert!(!inside_polygon(&p, &c));
}

#[test]
fn circle_intersect_check() {
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
    assert!(!circle_intersect(&c1, &poly1.0, &poly1.1));
    assert!(circle_intersect(&c2, &poly1.0, &poly1.1));
    assert!(circle_intersect(&c3, &poly1.0, &poly1.1));
    assert!(circle_intersect(&p, &poly1.0, &poly1.1));

    assert!(!circle_intersect(&c1, &poly2.0, &poly2.1));
    assert!(circle_intersect(&c2, &poly2.0, &poly2.1));
    assert!(circle_intersect(&c3, &poly2.0, &poly2.1));
    assert!(circle_intersect(&p, &poly2.0, &poly2.1));

    assert!(!circle_intersect(&c1, &poly3.0, &poly3.1));
    assert!(!circle_intersect(&c2, &poly3.0, &poly3.1));
    assert!(circle_intersect(&c3, &poly3.0, &poly3.1));
    assert!(!circle_intersect(&p, &poly3.0, &poly3.1));

}
