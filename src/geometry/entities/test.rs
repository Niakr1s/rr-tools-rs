use super::*;

#[test]
fn rect_add_point() {
    let mut r = Rect::new();

    r.add_point(&Point::new(1.0, 2.0, None));
    assert_eq!(r, Rect::from(1.0, 2., -2., -3.));

    r.add_point(&Point::new(0.0, 0.0, None));
    assert_eq!(r, Rect::from(1.0, 2., -2., -3.));

    r.add_point(&Point::new(-10.0, -11.0, None));
    assert_eq!(r, Rect::from(1.0, 2., -10., -11.));
}

#[test]
fn rect_add_circle() {
    let mut r = Rect::new();

    r.add_point(&Point::new(1.0, 0.5, Some(2.)));
    assert_eq!(r, Rect::from(3.0, 2.5, -1., -1.5));

    r.add_point(&Point::new(0.0, 0.0, Some(0.5)));
    assert_eq!(r, Rect::from(3.0, 2.5, -1., -1.5));

    r.add_point(&Point::new(2.0, 2.0, Some(1.)));
    assert_eq!(r, Rect::from(3.0, 3.0, -1., -1.5));
}