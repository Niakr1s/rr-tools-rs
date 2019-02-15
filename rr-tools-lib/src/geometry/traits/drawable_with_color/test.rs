use super::*;
use dxf::enums::AcadVersion;
use std::fs;
use std::path::Path;

#[test]
fn temp_dxf() {
    let point = Entity::from_point(Point::new(0., 0., None));
    let circle = Entity::from_point(Point::new(0., 0., Some(3.)));
    let closed_contur = Entity::from_contur(contur![
        Point::new(-1., -1., None),
        Point::new(-1., 1., None),
        Point::new(1., 1., None),
        Point::new(1., -1., None),
        Point::new(-1., -1., None)
    ])
    .unwrap(); // got closed rectangle
    let open_outer_contur = Entity::from_contur(contur![
        Point::new(-1.5, -1.5, None),
        Point::new(-1.5, 1.5, None),
        Point::new(1.5, 1.5, None),
        Point::new(1.5, -1.5, None)
    ])
    .unwrap();
    let open_inner_contur = Entity::from_contur(contur![
        Point::new(-0.5, -0.5, None),
        Point::new(-0.5, 0.5, None),
        Point::new(0.5, 0.5, None),
        Point::new(0.5, -0.5, None)
    ])
    .unwrap();

    let entities: Entities = vec![
        point,
        circle,
        closed_contur,
        open_inner_contur,
        open_outer_contur,
    ];

    let mut drawing = Drawing::default();

    entities.draw_with_color(&mut drawing, 2);

    drawing
        .save_file("temp.dxf")
        .expect("error while saving drawing");

    assert!(Drawing::load_file("temp.dxf").is_ok());
}
