use crate::geometry::checks::*;
use crate::geometry::entities::*;
use crate::geometry::traits::rectangable::Rectangable;

pub trait Relative: Rectangable {
    fn relate_entity(&self, entity: &Entity) -> Option<Relation>;
    fn relate_entities(&self, entities: &Entities) -> Option<Relation>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Relation {
    Inside,
    Intersect,
}

impl Relative for Entities {
    /// logic fully similar to Entities::relate_entities
    fn relate_entity(&self, entity: &Entity) -> Option<Relation> {
        let mut checks = vec![];
        for self_entity in self {
            let check = self_entity.relate_entity(entity);
            if let Some(Relation::Intersect) = check {
                debug!("intersect!");
                return Some(Relation::Intersect);
            };
            checks.push(check);
        }
        debug!("got checks: {:?}", checks);

        if checks.iter().all(|x| *x == None) {
            return None;
        };

        Some(Relation::Inside)
    }

    /// logic fully similar to Entities::relate_entity
    fn relate_entities(&self, entities: &Entities) -> Option<Relation> {
        let mut checks = vec![];
        for self_entity in self {
            let check = self_entity.relate_entities(entities);
            if let Some(Relation::Intersect) = check {
                debug!("intersect!");
                return Some(Relation::Intersect);
            };
            checks.push(check);
        }
        debug!("got checks: {:?}", checks);

        if checks.iter().all(|x| *x == None) {
            return None;
        };

        Some(Relation::Inside)
    }
}

impl Relative for Entity {
    fn relate_entity(&self, entity: &Entity) -> Option<Relation> {
        if self.can_not_intersect(entity) {
            return None;
        };

        match self {
            Entity::Point(ref self_point) => match entity {
                Entity::Point(ref other_point) => {
                    if circle_inside_circle(self_point, other_point) {
                        Some(Relation::Inside)
                    } else if circle_intersect_circle(self_point, other_point) {
                        Some(Relation::Intersect)
                    } else {
                        None
                    }
                }

                Entity::Contur(ref other_contur) => {
                    if circle_inside_contur(self_point, other_contur) {
                        return Some(Relation::Inside);
                    };

                    let other_points = &other_contur.points;

                    let mut other_first = other_points.first().unwrap();
                    for other_p in other_points {
                        if circle_relate_line(self_point, (other_first, other_p)) {
                            return Some(Relation::Intersect);
                        };
                        other_first = other_p;
                    }
                    None
                }
            },

            Entity::Contur(ref self_contur) => {
                fn inpolygon_switch(inpolygon: &mut bool, condition: bool) {
                    if *inpolygon {
                        *inpolygon &= condition;
                    }
                }

                fn intersect_switch(intersect: &mut bool, condition: bool) {
                    if !*intersect {
                        *intersect |= condition;
                    };
                }

                // true when ALL true, so we can not return from loops
                let mut inpolygon = true;
                // true when ANY true
                let mut intersect = false;

                let self_points = &self_contur.points;

                let mut self_first = self_points.first().unwrap();
                for self_p in self_points {
                    match entity {
                        Entity::Point(ref other_point) => {
                            if circle_inside_contur(other_point, self_contur) {
                                return Some(Relation::Intersect);
                            };
                            intersect_switch(
                                &mut intersect,
                                circle_relate_line(other_point, (self_first, self_p)),
                            );
                            inpolygon_switch(
                                &mut inpolygon,
                                circle_inside_circle(&self_p, other_point),
                            );
                        }

                        Entity::Contur(ref other_contur) => {
                            inpolygon_switch(
                                &mut inpolygon,
                                circle_inside_contur(self_p, other_contur),
                            );

                            // If other contur lies inside self contur -> self contur is intersecting it
                            let mut other_inpolygon = self_contur.is_closed();

                            let other_points = &other_contur.points;

                            let mut other_first = other_points.first().unwrap();
                            for other_p in other_points {
                                let self_segment = (self_first, self_p);
                                let other_segment = (other_first, other_p);
                                inpolygon_switch(
                                    &mut other_inpolygon,
                                    circle_inside_contur(other_p, self_contur),
                                );
                                if lines_intersect(self_segment, other_segment) {
                                    return Some(Relation::Intersect);
                                };
                                other_first = other_p;
                            }
                            if other_inpolygon {
                                intersect = true
                            };
                        }
                    }
                    self_first = self_p;
                }

                if inpolygon {
                    Some(Relation::Inside)
                } else if intersect {
                    Some(Relation::Intersect)
                } else {
                    None
                }
            }
        }
    }

    fn relate_entities(&self, entities: &Entities) -> Option<Relation> {
        let mut in_hole_counter = 0;
        debug!("\nentities len: {}", entities.len());
        for e in entities {
            let relate = self.relate_entity(e);
            debug!("got self.relate_entity {:?}", relate);
            match relate {
                Some(Relation::Intersect) => {
                    return Some(Relation::Intersect);
                }
                Some(Relation::Inside) => in_hole_counter += 1,
                None => continue,
            }
        }
        debug!("in hole_counter: {}", in_hole_counter);
        if in_hole_counter % 2 == 1 {
            return Some(Relation::Inside);
        };
        None
    }
}

#[cfg(test)]
mod test {
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

        assert_eq!(point.relate_entity(&point), Some(Relation::Intersect));
        assert_eq!(point.relate_entity(&circle), Some(Relation::Inside));
        assert_eq!(point.relate_entity(&closed_contur), Some(Relation::Inside));
        assert_eq!(point.relate_entity(&open_outer_contur), None);
        assert_eq!(point.relate_entity(&open_inner_contur), None);

        assert_eq!(circle.relate_entity(&point), Some(Relation::Intersect));
        assert_eq!(circle.relate_entity(&circle), Some(Relation::Intersect));
        assert_eq!(
            circle.relate_entity(&closed_contur),
            Some(Relation::Intersect)
        );
        assert_eq!(
            circle.relate_entity(&open_outer_contur),
            Some(Relation::Intersect)
        );
        assert_eq!(
            circle.relate_entity(&open_inner_contur),
            Some(Relation::Intersect)
        );

        assert_eq!(
            closed_contur.relate_entity(&point),
            Some(Relation::Intersect)
        );
        assert_eq!(closed_contur.relate_entity(&circle), Some(Relation::Inside));
        assert_eq!(
            closed_contur.relate_entity(&closed_contur),
            Some(Relation::Intersect)
        );
        assert_eq!(closed_contur.relate_entity(&open_outer_contur), None);
        assert_eq!(
            closed_contur.relate_entity(&open_inner_contur),
            Some(Relation::Intersect)
        );

        assert_eq!(open_outer_contur.relate_entity(&point), None);
        assert_eq!(
            open_outer_contur.relate_entity(&circle),
            Some(Relation::Inside)
        );
        assert_eq!(open_outer_contur.relate_entity(&closed_contur), None);
        assert_eq!(
            open_outer_contur.relate_entity(&open_outer_contur),
            Some(Relation::Intersect)
        );
        assert_eq!(open_outer_contur.relate_entity(&open_inner_contur), None);

        assert_eq!(open_inner_contur.relate_entity(&point), None);
        assert_eq!(
            open_inner_contur.relate_entity(&circle),
            Some(Relation::Inside)
        );
        assert_eq!(
            open_inner_contur.relate_entity(&closed_contur),
            Some(Relation::Inside)
        );
        assert_eq!(open_inner_contur.relate_entity(&open_outer_contur), None);
        assert_eq!(
            open_inner_contur.relate_entity(&open_inner_contur),
            Some(Relation::Intersect)
        );
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
        ])
        .unwrap(); // got closed rectangle
        let open_outer_contur = Entity::from_contur(contur![
            Point::new(0., -1.5, None),
            Point::new(0., 1.5, None),
            Point::new(3., 1.5, None),
            Point::new(3., -1.5, None)
        ])
        .unwrap();
        let open_inner_contur = Entity::from_contur(contur![
            Point::new(-0.5, -1.5, None),
            Point::new(-0.5, -0.5, None),
            Point::new(0.5, -0.5, None),
            Point::new(0.5, -1.5, None)
        ])
        .unwrap();

        assert_eq!(point.relate_entity(&point), Some(Relation::Intersect));
        assert_eq!(point.relate_entity(&circle), Some(Relation::Inside));
        assert_eq!(point.relate_entity(&closed_contur), Some(Relation::Inside));
        assert_eq!(
            point.relate_entity(&open_outer_contur),
            Some(Relation::Intersect)
        );
        assert_eq!(point.relate_entity(&open_inner_contur), None);

        assert_eq!(circle.relate_entity(&point), Some(Relation::Intersect));
        assert_eq!(circle.relate_entity(&circle), Some(Relation::Intersect));
        assert_eq!(
            circle.relate_entity(&closed_contur),
            Some(Relation::Intersect)
        );
        assert_eq!(
            circle.relate_entity(&open_outer_contur),
            Some(Relation::Intersect)
        );
        assert_eq!(
            circle.relate_entity(&open_inner_contur),
            Some(Relation::Intersect)
        );

        assert_eq!(
            closed_contur.relate_entity(&point),
            Some(Relation::Intersect)
        );
        assert_eq!(
            closed_contur.relate_entity(&circle),
            Some(Relation::Intersect)
        );
        assert_eq!(
            closed_contur.relate_entity(&closed_contur),
            Some(Relation::Intersect)
        );
        assert_eq!(
            closed_contur.relate_entity(&open_outer_contur),
            Some(Relation::Intersect)
        );
        assert_eq!(
            closed_contur.relate_entity(&open_inner_contur),
            Some(Relation::Intersect)
        );

        assert_eq!(
            open_outer_contur.relate_entity(&point),
            Some(Relation::Intersect)
        );
        assert_eq!(
            open_outer_contur.relate_entity(&circle),
            Some(Relation::Intersect)
        );
        assert_eq!(
            open_outer_contur.relate_entity(&closed_contur),
            Some(Relation::Intersect)
        );
        assert_eq!(
            open_outer_contur.relate_entity(&open_outer_contur),
            Some(Relation::Intersect)
        );
        assert_eq!(
            open_outer_contur.relate_entity(&open_inner_contur),
            Some(Relation::Intersect)
        );

        assert_eq!(open_inner_contur.relate_entity(&point), None);
        assert_eq!(
            open_inner_contur.relate_entity(&circle),
            Some(Relation::Inside)
        );
        assert_eq!(
            open_inner_contur.relate_entity(&closed_contur),
            Some(Relation::Intersect)
        );
        assert_eq!(
            open_inner_contur.relate_entity(&open_outer_contur),
            Some(Relation::Intersect)
        );
        assert_eq!(
            open_inner_contur.relate_entity(&open_inner_contur),
            Some(Relation::Intersect)
        );
    }

    #[test]
    fn relate_entities_fig3() {
        let outer = Entity::from_contur(contur![
            Point::new(-9., -9., None),
            Point::new(-9., 8., None),
            Point::new(10., 10., None),
            Point::new(11., -6., None),
            Point::new(-4., -12., None),
            Point::new(-9., -9., None)
        ])
        .unwrap();
        let inner1 = Entity::from_contur(contur![
            Point::new(-6., 0., None),
            Point::new(-9., 8., None),
            Point::new(-2., 0., None),
            Point::new(3., -6., None),
            Point::new(-5., -7., None),
            Point::new(-6., 0., None)
        ])
        .unwrap();
        let inner1_inner = Entity::from_contur(contur![
            Point::new(-4., -2., None),
            Point::new(-2., -1., None),
            Point::new(0., -5., None),
            Point::new(-4., -5., None),
            Point::new(-4., -2., None)
        ])
        .unwrap();
        let inner2 = Entity::from_contur(contur![
            Point::new(3., 2., None),
            Point::new(7., 2., None),
            Point::new(9., -1., None),
            Point::new(4., -3., None),
            Point::new(3., 2., None)
        ])
        .unwrap();
        let entities: Entities = vec![outer, inner1, inner1_inner, inner2];

        let red_line = Entity::from_contur(contur![
            Point::new(-3., -3., None),
            Point::new(-2., -3., None),
            Point::new(-2., -4., None),
            Point::new(-3., -4., None)
        ])
        .unwrap();

        assert_eq!(red_line.relate_entities(&entities), Some(Relation::Inside));

        let green_line = Entity::from_contur(contur![
            Point::new(-1., -6., None),
            Point::new(-4., -6., None),
            Point::new(-5., -4., None),
            Point::new(-5., -1., None),
            Point::new(-3., -1., None)
        ])
        .unwrap();

        assert_eq!(green_line.relate_entities(&entities), None);

        let red_circle = Entity::from_point(Point::new(-2.5, -3.5, Some(1.)));

        assert_eq!(
            red_circle.relate_entities(&entities),
            Some(Relation::Inside)
        );

        let blue_circle = Entity::from_point(Point::new(6., 0., Some(4.)));

        assert_eq!(
            blue_circle.relate_entities(&entities),
            Some(Relation::Intersect)
        );

        let green_circle = Entity::from_point(Point::new(6., 0., Some(1.)));

        assert_eq!(green_circle.relate_entities(&entities), None);

        let blue_line = Entity::from_contur(contur![
            Point::new(6., -3., None),
            Point::new(6., -2., None)
        ])
        .unwrap();

        assert_eq!(
            blue_line.relate_entities(&entities),
            Some(Relation::Intersect)
        );

        let faraway_line = Entity::from_contur(contur![
            Point::new(1000., 1000., None),
            Point::new(2000., 2000., None)
        ])
        .unwrap();

        assert_eq!(faraway_line.relate_entities(&entities), None);
    }

    #[test]
    fn entities_relate_entity_fig4() {
        let mut mydxf_mock = vec![Entity::from_contur(contur![
            Point::new(-4., -9., None),
            Point::new(-0.9, -7.2, None),
            Point::new(-0.9, -10., None),
            Point::new(-3., -10., None)
        ])
        .unwrap()];

        let outer = Entity::from_contur(contur![
            Point::new(-8., -1., None),
            Point::new(14., -1., None),
            Point::new(7., -20., None),
            Point::new(-8., -15., None),
            Point::new(-8., -1., None)
        ])
        .unwrap();
        assert_eq!(mydxf_mock.relate_entity(&outer), Some(Relation::Inside));

        let mut rrxml_mock = vec![outer];

        rrxml_mock.push(
            Entity::from_contur(contur![
                Point::new(-6., -5., None),
                Point::new(2.2, -4.3, None),
                Point::new(-0., -14., None),
                Point::new(-6., -10., None),
                Point::new(-6., -5., None)
            ])
            .unwrap(),
        );
        assert_eq!(mydxf_mock.relate_entities(&rrxml_mock), None);

        rrxml_mock.push(
            Entity::from_contur(contur![
                Point::new(-4., -6., None),
                Point::new(1., -6., None),
                Point::new(-1., -12., None),
                Point::new(-5., -10., None),
                Point::new(-4., -6., None)
            ])
            .unwrap(),
        );
        assert_eq!(
            mydxf_mock.relate_entities(&rrxml_mock),
            Some(Relation::Inside)
        );

        rrxml_mock.push(
            Entity::from_contur(contur![
                Point::new(5., -4., None),
                Point::new(11., -4., None),
                Point::new(9., -11., None),
                Point::new(4., -11., None),
                Point::new(5., -4., None)
            ])
            .unwrap(),
        );
        assert_eq!(
            mydxf_mock.relate_entities(&rrxml_mock),
            Some(Relation::Inside)
        );

        // pushing some circle outside contur (but within "hole" contur)
        mydxf_mock.push(Entity::from_point(Point::new(8., -7., Some(1.))));
        assert_eq!(
            mydxf_mock.relate_entities(&rrxml_mock),
            Some(Relation::Inside)
        );

        // pushing some circle outside outer contur
        mydxf_mock.pop();
        mydxf_mock.push(Entity::from_point(Point::new(23., -7., Some(1.))));
        assert_eq!(
            mydxf_mock.relate_entities(&rrxml_mock),
            Some(Relation::Inside)
        );

        // pushing some circle inside outer
        mydxf_mock.pop();
        mydxf_mock.push(Entity::from_point(Point::new(5., -15., Some(1.))));
        assert_eq!(
            mydxf_mock.relate_entities(&rrxml_mock),
            Some(Relation::Inside)
        );

        let faraway_line = vec![Entity::from_contur(contur![
            Point::new(20., 20., None),
            Point::new(20., 20., None)
        ])
        .unwrap()];
        assert_eq!(faraway_line.relate_entities(&rrxml_mock), None);
    }

}
