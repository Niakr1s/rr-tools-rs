use crate::geometry::traits::relative::*;
use super::*;

#[test]
fn entity_relate_entity_fig1() {
    let point = Entity::from_point(Point::new(0., 0., None));
    let circle = Entity::from_point(Point::new(0., 0., Some(3.)));
    let closed_contur = Entity::from_contur(contur![
        Point::new(-1., -1., None),
        Point::new(-1., 1., None),
        Point::new(1., 1., None),
        Point::new(1., -1., None),
        Point::new(-1., -1., None)
    ]).unwrap();  // got closed rectangle
    let open_outer_contur = Entity::from_contur(contur![
        Point::new(-1.5, -1.5, None),
        Point::new(-1.5, 1.5, None),
        Point::new(1.5, 1.5, None),
        Point::new(1.5, -1.5, None)
    ]).unwrap();
    let open_inner_contur = Entity::from_contur(contur![
        Point::new(-0.5, -0.5, None),
        Point::new(-0.5, 0.5, None),
        Point::new(0.5, 0.5, None),
        Point::new(0.5, -0.5, None)
    ]).unwrap();

    assert_eq!(point.relate_entity(&point), Some(Relation::Intersect));
    assert_eq!(point.relate_entity(&circle), Some(Relation::Inside));
    assert_eq!(point.relate_entity(&closed_contur), Some(Relation::Inside));
    assert_eq!(point.relate_entity(&open_outer_contur), None);
    assert_eq!(point.relate_entity(&open_inner_contur), None);

    assert_eq!(circle.relate_entity(&point), Some(Relation::Inside));
    assert_eq!(circle.relate_entity(&circle), Some(Relation::Intersect));
    assert_eq!(circle.relate_entity(&closed_contur), Some(Relation::Inside));
    assert_eq!(circle.relate_entity(&open_outer_contur), Some(Relation::Inside));
    assert_eq!(circle.relate_entity(&open_inner_contur), Some(Relation::Inside));

    assert_eq!(closed_contur.relate_entity(&point), Some(Relation::Inside));
    assert_eq!(closed_contur.relate_entity(&circle), Some(Relation::Inside));
    assert_eq!(closed_contur.relate_entity(&closed_contur), Some(Relation::Intersect));
    assert_eq!(closed_contur.relate_entity(&open_outer_contur), None);
    assert_eq!(closed_contur.relate_entity(&open_inner_contur), Some(Relation::Inside));

    assert_eq!(open_outer_contur.relate_entity(&point), None);
    assert_eq!(open_outer_contur.relate_entity(&circle), Some(Relation::Inside));
    assert_eq!(open_outer_contur.relate_entity(&closed_contur), None);
    assert_eq!(open_outer_contur.relate_entity(&open_outer_contur), Some(Relation::Intersect));
    assert_eq!(open_outer_contur.relate_entity(&open_inner_contur), None);

    assert_eq!(open_inner_contur.relate_entity(&point), None);
    assert_eq!(open_inner_contur.relate_entity(&circle), Some(Relation::Inside));
    assert_eq!(open_inner_contur.relate_entity(&closed_contur), Some(Relation::Inside));
    assert_eq!(open_inner_contur.relate_entity(&open_outer_contur), None);
    assert_eq!(open_inner_contur.relate_entity(&open_inner_contur), Some(Relation::Intersect));
}

#[test]
fn entity_relate_entity_fig2() {
    let point = Entity::from_point(Point::new(0., 0., None));
    let circle = Entity::from_point(Point::new(0., -2., Some(3.)));
    let closed_contur = Entity::from_contur(contur![
        Point::new(-1., -1., None),
        Point::new(-1., 1., None),
        Point::new(1., 1., None),
        Point::new(1., -1., None),
        Point::new(-1., -1., None)
    ]).unwrap();  // got closed rectangle
    let open_outer_contur = Entity::from_contur(contur![
        Point::new(0., -1.5, None),
        Point::new(0., 1.5, None),
        Point::new(3., 1.5, None),
        Point::new(3., -1.5, None)
    ]).unwrap();
    let open_inner_contur = Entity::from_contur(contur![
        Point::new(-0.5, -1.5, None),
        Point::new(-0.5, -0.5, None),
        Point::new(0.5, -0.5, None),
        Point::new(0.5, -1.5, None)
    ]).unwrap();

    assert_eq!(point.relate_entity(&point), Some(Relation::Intersect));
    assert_eq!(point.relate_entity(&circle), Some(Relation::Inside));
    assert_eq!(point.relate_entity(&closed_contur), Some(Relation::Inside));
    assert_eq!(point.relate_entity(&open_outer_contur), Some(Relation::Intersect));
    assert_eq!(point.relate_entity(&open_inner_contur), None);

    assert_eq!(circle.relate_entity(&point), Some(Relation::Inside));
    assert_eq!(circle.relate_entity(&circle), Some(Relation::Intersect));
    assert_eq!(circle.relate_entity(&closed_contur), Some(Relation::Intersect));
    assert_eq!(circle.relate_entity(&open_outer_contur), Some(Relation::Intersect));
    assert_eq!(circle.relate_entity(&open_inner_contur), Some(Relation::Inside));

    assert_eq!(closed_contur.relate_entity(&point), Some(Relation::Inside));
    assert_eq!(closed_contur.relate_entity(&circle), Some(Relation::Intersect));
    assert_eq!(closed_contur.relate_entity(&closed_contur), Some(Relation::Intersect));
    assert_eq!(closed_contur.relate_entity(&open_outer_contur), Some(Relation::Intersect));
    assert_eq!(closed_contur.relate_entity(&open_inner_contur), Some(Relation::Intersect));

    assert_eq!(open_outer_contur.relate_entity(&point), Some(Relation::Intersect));
    assert_eq!(open_outer_contur.relate_entity(&circle), Some(Relation::Intersect));
    assert_eq!(open_outer_contur.relate_entity(&closed_contur), Some(Relation::Intersect));
    assert_eq!(open_outer_contur.relate_entity(&open_outer_contur), Some(Relation::Intersect));
    assert_eq!(open_outer_contur.relate_entity(&open_inner_contur), Some(Relation::Intersect));

    assert_eq!(open_inner_contur.relate_entity(&point), None);
    assert_eq!(open_inner_contur.relate_entity(&circle), Some(Relation::Inside));
    assert_eq!(open_inner_contur.relate_entity(&closed_contur), Some(Relation::Intersect));
    assert_eq!(open_inner_contur.relate_entity(&open_outer_contur), Some(Relation::Intersect));
    assert_eq!(open_inner_contur.relate_entity(&open_inner_contur), Some(Relation::Intersect));
}

#[test]
fn eq_points_ref() {
    assert_eq!(&Point::new(1.0, 1.0, Some(1.0)), &Point::new(1.0, 1.0, Some(1.0)))
}

#[test]
fn intersect_switch_ok() {
    let mut intersect = false;
    intersect_switch(&mut intersect, true);
    assert_eq!(intersect, true);
}