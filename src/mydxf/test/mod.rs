use super::*;

const P1: Point = Point {
    x: 1.5,
    y: 1.5,
    r: None,
};
const P2: Point = Point {
    x: 2.5,
    y: 2.5,
    r: None,
};
const P3: Point = Point {
    x: 2.0,
    y: -3.0,
    r: None,
};

#[test]
fn triangle_polyline() {
    let path = "src/mydxf/test/dxfs/triangle_polyline.dxf";
    let my_dxf = MyDxf::from_file(path);
    assert!(my_dxf.is_ok(), "test file {} open error", path);
    let my_dxf = my_dxf.unwrap();

    let mut contur = Contur::new();
    contur.add(P1.clone());
    contur.add(P2.clone());
    contur.add(P3.clone());
    contur.add(P1.clone());
    assert_eq!(my_dxf.entities, vec![Entity::Contur(contur),]);
}

#[test]
fn triangle_line() {
    let path = "src/mydxf/test/dxfs/triangle_line.dxf";
    let my_dxf = MyDxf::from_file(path);
    assert!(my_dxf.is_ok(), "test file {} open error", path);
    let my_dxf = my_dxf.unwrap();

    let mut contur1 = Contur::new();
    contur1.add(P1.clone());
    contur1.add(P2.clone());
    let mut contur2 = Contur::new();
    contur2.add(P2.clone());
    contur2.add(P3.clone());
    let mut contur3 = Contur::new();
    contur3.add(P3.clone());
    contur3.add(P1.clone());

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
    let path = "src/mydxf/test/dxfs/circle.dxf";
    let my_dxf = MyDxf::from_file(path);
    assert!(my_dxf.is_ok(), "test file {} open error", path);
    let my_dxf = my_dxf.unwrap();

    assert_eq!(
        my_dxf.entities,
        vec![Entity::Point(Point {
            x: 1.5,
            y: 1.5,
            r: Some(1.5),
        }),]
    );
}

#[test]
fn point() {
    let path = "src/mydxf/test/dxfs/point.dxf";
    let my_dxf = MyDxf::from_file(path);
    assert!(my_dxf.is_ok(), "test file {} open error", path);
    let my_dxf = my_dxf.unwrap();

    assert_eq!(my_dxf.entities, vec![Entity::Point(P1.clone()),]);
}
