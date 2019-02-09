use crate::geometry::traits::relative::*;
use super::*;

/// For tests with fig* at the end, see drawing from folder \test_files\explanation\

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

    assert_eq!(circle.relate_entity(&point), Some(Relation::Intersect));
    assert_eq!(circle.relate_entity(&circle), Some(Relation::Intersect));
    assert_eq!(circle.relate_entity(&closed_contur), Some(Relation::Intersect));
    assert_eq!(circle.relate_entity(&open_outer_contur), Some(Relation::Intersect));
    assert_eq!(circle.relate_entity(&open_inner_contur), Some(Relation::Intersect));

    assert_eq!(closed_contur.relate_entity(&point), Some(Relation::Intersect));
    assert_eq!(closed_contur.relate_entity(&circle), Some(Relation::Inside));
    assert_eq!(closed_contur.relate_entity(&closed_contur), Some(Relation::Intersect));
    assert_eq!(closed_contur.relate_entity(&open_outer_contur), None);
    assert_eq!(closed_contur.relate_entity(&open_inner_contur), Some(Relation::Intersect));

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

    assert_eq!(circle.relate_entity(&point), Some(Relation::Intersect));
    assert_eq!(circle.relate_entity(&circle), Some(Relation::Intersect));
    assert_eq!(circle.relate_entity(&closed_contur), Some(Relation::Intersect));
    assert_eq!(circle.relate_entity(&open_outer_contur), Some(Relation::Intersect));
    assert_eq!(circle.relate_entity(&open_inner_contur), Some(Relation::Intersect));

    assert_eq!(closed_contur.relate_entity(&point), Some(Relation::Intersect));
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

#[test]
fn relate_entities_fig3() {
    let outer = Entity::from_contur(contur![
        Point::new(-9.,-9.,None),
        Point::new(-9.,8.,None),
        Point::new(10.,10.,None),
        Point::new(11.,-6.,None),
        Point::new(-4.,-12.,None),
        Point::new(-9.,-9.,None)
    ]).unwrap();
    let inner1 = Entity::from_contur(contur![
        Point::new(-6.,0.,None),
        Point::new(-9.,8.,None),
        Point::new(-2.,0.,None),
        Point::new(3.,-6.,None),
        Point::new(-5.,-7.,None),
        Point::new(-6.,0.,None)
    ]).unwrap();
    let inner1_inner = Entity::from_contur(contur![
        Point::new(-4.,-2.,None),
        Point::new(-2.,-1.,None),
        Point::new(0.,-5.,None),
        Point::new(-4.,-5.,None),
        Point::new(-4.,-2.,None)
    ]).unwrap();
    let inner2 = Entity::from_contur(contur![
        Point::new(3.,2.,None),
        Point::new(7.,2.,None),
        Point::new(9.,-1.,None),
        Point::new(4.,-3.,None),
        Point::new(3.,2.,None)
    ]).unwrap();
    let entities: Entities = vec![outer, inner1, inner1_inner, inner2];

    let red_line = Entity::from_contur(contur![
        Point::new(-3.,-3.,None),
        Point::new(-2.,-3.,None),
        Point::new(-2.,-4.,None),
        Point::new(-3.,-4.,None)
    ]).unwrap();

    assert_eq!(red_line.relate_entities(&entities), Some(Relation::Inside));

    let green_line = Entity::from_contur(contur![
        Point::new(-1.,-6.,None),
        Point::new(-4.,-6.,None),
        Point::new(-5.,-4.,None),
        Point::new(-5.,-1.,None),
        Point::new(-3.,-1.,None)
    ]).unwrap();

    assert_eq!(green_line.relate_entities(&entities), None);

    let red_circle = Entity::from_point(Point::new(-2.5, -3.5, Some(1.)));

    assert_eq!(red_circle.relate_entities(&entities), Some(Relation::Inside));

    let blue_circle = Entity::from_point(Point::new(6., 0., Some(4.)));

    assert_eq!(blue_circle.relate_entities(&entities), Some(Relation::Intersect));

    let green_circle = Entity::from_point(Point::new(6., 0., Some(1.)));

    assert_eq!(green_circle.relate_entities(&entities), None);

    let blue_line = Entity::from_contur(contur![
        Point::new(6.,-3.,None),
        Point::new(6.,-2.,None)
    ]).unwrap();

    assert_eq!(blue_line.relate_entities(&entities), Some(Relation::Intersect));
}
