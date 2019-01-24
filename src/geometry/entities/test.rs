use super::*;

#[test]
fn entity_intersect_entity() {
    let point = Entity::from_point(Point::new(0., 0., None));
    let circle = Entity::from_point(Point::new(0., 0., Some(3.)));
    let mut contur = Contur::new();
    contur.add(Point::new(-1., -1., None));
    contur.add(Point::new(-1., 1., None));
    contur.add(Point::new(1., 1., None));
    contur.add(Point::new(1., -1., None));
    contur.add(Point::new(-1., -1., None));
    let closed_contur = Entity::from_contur(contur.clone()).unwrap();
}