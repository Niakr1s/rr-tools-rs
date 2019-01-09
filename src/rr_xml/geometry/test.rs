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
