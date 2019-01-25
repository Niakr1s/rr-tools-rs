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

    assert!(point.relate_entity(&point));
    assert!(point.relate_entity(&circle));
    assert!(point.relate_entity(&closed_contur));
    assert!(!point.relate_entity(&open_outer_contur));
    assert!(!point.relate_entity(&open_inner_contur));

    assert!(circle.relate_entity(&point));
    assert!(circle.relate_entity(&circle));
    assert!(circle.relate_entity(&closed_contur));
    assert!(circle.relate_entity(&open_outer_contur));
    assert!(circle.relate_entity(&open_inner_contur));

    assert!(closed_contur.relate_entity(&point));
    assert!(closed_contur.relate_entity(&circle));
    assert!(closed_contur.relate_entity(&closed_contur));
    assert!(!closed_contur.relate_entity(&open_outer_contur));
    assert!(closed_contur.relate_entity(&open_inner_contur));

    assert!(!open_outer_contur.relate_entity(&point));
    assert!(open_outer_contur.relate_entity(&circle));
    assert!(!open_outer_contur.relate_entity(&closed_contur));
    assert!(open_outer_contur.relate_entity(&open_outer_contur));
    assert!(!open_outer_contur.relate_entity(&open_inner_contur));

    assert!(!open_inner_contur.relate_entity(&point));
    assert!(open_inner_contur.relate_entity(&circle));
    assert!(open_inner_contur.relate_entity(&closed_contur));
    assert!(!open_inner_contur.relate_entity(&open_outer_contur));
    assert!(open_inner_contur.relate_entity(&open_inner_contur));
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

    assert!(point.relate_entity(&point));
    assert!(point.relate_entity(&circle));
    assert!(point.relate_entity(&closed_contur));
    assert!(point.relate_entity(&open_outer_contur));
    assert!(!point.relate_entity(&open_inner_contur));

    assert!(circle.relate_entity(&point));
    assert!(circle.relate_entity(&circle));
    assert!(circle.relate_entity(&closed_contur));
    assert!(circle.relate_entity(&open_outer_contur));
    assert!(circle.relate_entity(&open_inner_contur));

    assert!(closed_contur.relate_entity(&point));
    assert!(closed_contur.relate_entity(&circle));
    assert!(closed_contur.relate_entity(&closed_contur));
    assert!(closed_contur.relate_entity(&open_outer_contur));
    assert!(closed_contur.relate_entity(&open_inner_contur));

    assert!(open_outer_contur.relate_entity(&point));
    assert!(open_outer_contur.relate_entity(&circle));
    assert!(open_outer_contur.relate_entity(&closed_contur));
    assert!(open_outer_contur.relate_entity(&open_outer_contur));
    assert!(open_outer_contur.relate_entity(&open_inner_contur));

    assert!(!open_inner_contur.relate_entity(&point));
    assert!(open_inner_contur.relate_entity(&circle));
    assert!(open_inner_contur.relate_entity(&closed_contur));
    assert!(open_inner_contur.relate_entity(&open_outer_contur));
    assert!(open_inner_contur.relate_entity(&open_inner_contur));
}

#[test]
fn eq_points_ref() {
    assert_eq!(&Point::new(1.0, 1.0, Some(1.0)), &Point::new(1.0, 1.0, Some(1.0)))
}