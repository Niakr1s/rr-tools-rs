use super::*;

#[test]
fn rect_add_point() {
    let mut r = Rect::new();

    r.add_point(&Point::new(1.0, 2.0, None));
    assert_eq!(r, Rect::from(1.0, 2., 1., 2.).unwrap());

    r.add_point(&Point::new(0.0, 0.0, None));
    assert_eq!(r, Rect::from(1.0, 2., 0., 0.).unwrap());

    r.add_point(&Point::new(-10.0, -11.0, None));
    assert_eq!(r, Rect::from(1.0, 2., -10., -11.).unwrap());
}

#[test]
fn rect_add_circle() {
    let mut r = Rect::new();

    r.add_point(&Point::new(1.0, 0.5, Some(2.)));
    assert_eq!(r, Rect::from(3.0, 2.5, -1., -1.5).unwrap());

    r.add_point(&Point::new(0.0, 0.0, Some(0.5)));
    assert_eq!(r, Rect::from(3.0, 2.5, -1., -1.5).unwrap());

    r.add_point(&Point::new(2.0, 2.0, Some(1.)));
    assert_eq!(r, Rect::from(3.0, 3.0, -1., -1.5).unwrap());
}

#[test]
fn rect_add_rect() {
    let mut r = Rect::from(3., 3., -2., -2.).unwrap();
    let other = Rect::from(4., 4., -3., -3.).unwrap();
    r.add_rect(&other);
    assert_eq!(r, other);

    let mut r = Rect::from(3., 3., -2., -2.).unwrap();
    let other = Rect::from(5., 4., -1., -0.).unwrap();
    r.add_rect(&other);
    assert_eq!(r, Rect::from(5., 4., -2., -2.).unwrap());

    let mut r = Rect::from(3., 3., -2., -2.).unwrap();
    let other = Rect::from(5., 4., -3., -4.).unwrap();
    r.add_rect(&other);
    assert_eq!(r, Rect::from(5., 4., -3., -4.).unwrap());
}