use super::*;

#[test]
fn is_intersect_check1() {
    // should be false
    let seg1 = (
        Point {
            x: 1.,
            y: 1.,
            r: 0.,
        },
        Point {
            x: 10.,
            y: 1.,
            r: 0.,
        },
    );
    let seg2 = (
        Point {
            x: 1.,
            y: 2.,
            r: 0.,
        },
        Point {
            x: 10.,
            y: 2.,
            r: 0.,
        },
    );
    assert!(!is_intersect(&seg1, &seg2));
}

#[test]
fn is_intersect_check2() {
    // should be true
    let seg1 = (
        Point {
            x: 10.,
            y: 0.,
            r: 0.,
        },
        Point {
            x: 0.,
            y: 10.,
            r: 0.,
        },
    );
    let seg2 = (
        Point {
            x: 0.,
            y: 0.,
            r: 0.,
        },
        Point {
            x: 10.,
            y: 10.,
            r: 0.,
        },
    );
    assert!(is_intersect(&seg1, &seg2));
}
#[test]
fn is_intersect_check3() {
    // should be false
    let seg1 = (
        Point {
            x: -5.,
            y: -5.,
            r: 0.,
        },
        Point {
            x: 0.,
            y: 0.,
            r: 0.,
        },
    );
    let seg2 = (
        Point {
            x: 1.,
            y: 1.,
            r: 0.,
        },
        Point {
            x: 10.,
            y: 10.,
            r: 0.,
        },
    );
    assert!(!is_intersect(&seg1, &seg2));
}

#[test]
fn inside_polygon_check1() {
    let p = Point {
        x: 1.,
        y: 1.,
        r: 0.,
    };
    let c = Contur {
        points: vec![
            Point {
                x: -2.,
                y: -2.,
                r: 0.,
            },
            Point {
                x: 2.,
                y: -2.,
                r: 0.,
            },
            Point {
                x: 1.,
                y: 2.,
                r: 0.,
            },
            Point {
                x: -2.,
                y: -2.,
                r: 0.,
            },
        ],
    };
    assert!(inside_polygon(&p, &c));
}

#[test]
fn inside_polygon_check2() {
    let p = Point {
        x: 1.,
        y: 1.,
        r: 0.,
    };
    let c = Contur {
        points: vec![
            Point {
                x: -2.,
                y: -2.,
                r: 0.,
            },
            Point {
                x: 2.,
                y: -2.,
                r: 0.,
            },
            Point {
                x: 2.,
                y: 0.,
                r: 0.,
            },
            Point {
                x: -2.,
                y: -2.,
                r: 0.,
            },
        ],
    };
    assert!(!inside_polygon(&p, &c));
}

#[test]
fn circle_intersect_check() {
    let c1 = Point {
        x: 0.,
        y: 0.,
        r: 1.,
    };
    let c2 = Point {
        x: 0.,
        y: 0.,
        r: 2.,
    };
    let c3 = Point {
        x: 0.,
        y: 0.,
        r: 3.,
    };
    let poly1 = (
        Point {
            x: 2.,
            y: -1.,
            r: 0.,
        },
        Point {
            x: 2.,
            y: 1.,
            r: 0.,
        },
    );
    let poly2 = (
        Point {
            x: 2.,
            y: -1.,
            r: 0.,
        },
        Point {
            x: 2.,
            y: 0.,
            r: 0.,
        },
    );
    let poly3 = (
        Point {
            x: 2.,
            y: -1.,
            r: 0.,
        },
        Point {
            x: 2.,
            y: -0.001,
            r: 0.,
        },
    );
    assert!(!circle_intersect(&c1, &poly1.0, &poly1.1));
    assert!(circle_intersect(&c2, &poly1.0, &poly1.1));
    assert!(circle_intersect(&c3, &poly1.0, &poly1.1));

    assert!(!circle_intersect(&c1, &poly2.0, &poly2.1));
    assert!(circle_intersect(&c2, &poly2.0, &poly2.1));
    assert!(circle_intersect(&c3, &poly2.0, &poly2.1));

    assert!(!circle_intersect(&c1, &poly3.0, &poly3.1));
    assert!(!circle_intersect(&c2, &poly3.0, &poly3.1));
    assert!(circle_intersect(&c3, &poly3.0, &poly3.1));
}
