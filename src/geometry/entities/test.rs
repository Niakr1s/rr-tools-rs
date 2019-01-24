use super::*;

#[test]
fn entity_intersect_entity() {
    let point = Entity::from_point(Point::new(0., 0., None));
    let circle = Entity::from_point(Point::new(0., 0., Some(3.)));
    let contur = contur![
        Point::new(-1., -1., None),
        Point::new(-1., 1., None),
        Point::new(1., 1., None),
        Point::new(1., -1., None),
        Point::new(-1., -1., None)
    ];
    let closed_contur = Entity::from_contur(contur.clone()).unwrap();
}