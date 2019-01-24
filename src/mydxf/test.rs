use super::*;

/// In this test module result should be with reversed coordinates

const P1: Point = Point { x: 1.5, y: 1.5, r: None };
const P2: Point = Point { x: 2.5, y: 2.5, r: None };
const P3: Point = Point { x: -3.0, y: 2.0, r: None };

#[test]
fn triangle_polyline() {
    let path = r"src\test_files\dxfs\triangle_polyline.dxf";
    let my_dxf = MyDxf::from_file(path);
    assert!(my_dxf.is_ok(), "test file {} open error", path);
    let my_dxf = my_dxf.unwrap();

    let contur = contur![P1.clone(), P2.clone(), P3.clone(), P1.clone()];
    assert_eq!(my_dxf.entities, vec![Entity::Contur(contur),]);
}

#[test]
fn triangle_line() {
    let path = r"src\test_files\dxfs\triangle_line.dxf";
    let my_dxf = MyDxf::from_file(path);
    assert!(my_dxf.is_ok(), "test file {} open error", path);
    let my_dxf = my_dxf.unwrap();

    let contur1 = contur![P1.clone(), P2.clone()];
    let contur2 = contur![P2.clone(), P3.clone()];
    let contur3 = contur![P3.clone(), P1.clone()];

    assert_eq!(
        my_dxf.entities,
        vec![
            Entity::Contur(contur1),
            Entity::Contur(contur2),
            Entity::Contur(contur3),
        ]
    );
}

#[test]
fn circle() {
    let path = r"src\test_files\dxfs\circle.dxf";
    let my_dxf = MyDxf::from_file(path);
    assert!(my_dxf.is_ok(), "test file {} open error", path);
    let my_dxf = my_dxf.unwrap();

    assert_eq!(
        my_dxf.entities,
        vec![Entity::Point(Point::new(1.5, 1.5, Some(1.5)))]
    );
}

#[test]
fn point() {
    let path = r"src\test_files\dxfs\point.dxf";
    let my_dxf = MyDxf::from_file(path);
    assert!(my_dxf.is_ok(), "test file {} open error", path);
    let my_dxf = my_dxf.unwrap();

    assert_eq!(my_dxf.entities, vec![Entity::Point(P1.clone())]);
}