use crate::geometry::checks::*;
use crate::geometry::traits::rectangable::*;
use crate::geometry::traits::relative::*;
use dxf::entities::Circle as DxfCircle;
use dxf::Point as DxfPoint;

pub type Entities = Vec<Entity>;

impl Rectangable for Entities {
    fn rect(&self) -> Rect {
        let mut rect = Rect::new();
        for e in self {
            rect.add(e);
        };
        rect
    }
}

impl Relative for Entities {
    fn relate_entity(&self, entity: &Entity) -> Option<Relation> {
        let mut checks = vec![];
        for self_entity in self {
            let check = self_entity.relate_entity(entity);
            if let Some(Relation::Intersect) = check {
                debug!("intersect!");
                return Some(Relation::Intersect);
            };
            checks.push(check);
        };
        debug!("got checks: {:?}", checks);

        if checks.iter().all(|x| *x == None) { return None };

        Some(Relation::Inside)
    }

    fn relate_entities(&self, entities: &Entities) -> Option<Relation> {
        let mut checks = vec![];
        for self_entity in self {
            let check = self_entity.relate_entities(entities);
            if let Some(Relation::Intersect) = check {
                debug!("intersect!");
                return Some(Relation::Intersect);
            };
            checks.push(check);
        };
        debug!("got checks: {:?}", checks);

        if checks.iter().all(|x| *x == None) { return None };

        Some(Relation::Inside)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Entity {
    Contur(Contur),
    Point(Point),
}

impl Entity {
    pub fn from_point(p: Point) -> Entity {
        Entity::Point(p)
    }

    /// Can return Some(Entity::Contur), Some(Entity::Point) and None
    pub fn from_contur(c: Contur) -> Option<Entity> {
        let mut c = c;
        match c.len() {
            0 => None,
            1 => Some(Entity::Point(c.points.pop().unwrap())),
            _ => Some(Entity::Contur(c)),
        }
    }
}

impl Rectangable for Entity {
    fn rect(&self) -> Rect {
        let mut rect = Rect::new();
        match self {
            Entity::Contur(ref c) => rect.add(c),
            Entity::Point(ref p) => rect.add(p),
        }
        rect
    }
}

impl Relative for Entity {
    fn relate_entity(&self, entity: &Entity) -> Option<Relation> {
        if self.can_not_intersect(entity) { return None };

        match self {
            Entity::Point(ref self_point) => match entity {
                Entity::Point(ref other_point) => {
                    match circle_inside_circle(self_point, other_point) {
                        true => Some(Relation::Inside),
                        false => match circle_intersect_circle(self_point, other_point) {
                            true => Some(Relation::Intersect),
                            false => None,
                        },
                    }
                },

                Entity::Contur(ref other_contur) => {
                    if circle_inside_contur(self_point, other_contur) { return Some(Relation::Inside) };

                    let other_points = &other_contur.points;

                    let mut other_first = other_points.first().unwrap();
                    for other_p in other_points {
                        if circle_relate_line(self_point, (other_first, other_p)) { return Some(Relation::Intersect) };
                        other_first = other_p;
                    };
                    None
                },
            },

            Entity::Contur(ref self_contur) => {
                // true when ALL true, so we can not return from loops
                let mut inpolygon = true;
                // true when ANY true
                let mut intersect = false;

                let self_points = &self_contur.points;

                let mut self_first = self_points.first().unwrap();
                for self_p in self_points {
                    match entity {
                        Entity::Point(ref other_point) => {
                            if circle_inside_contur(other_point, self_contur) { return Some(Relation::Intersect) };
                            intersect_switch(&mut intersect, circle_relate_line(other_point, (self_first, self_p)));
                            inpolygon_switch(&mut inpolygon, circle_inside_circle(&self_p, other_point));
                        },

                        Entity::Contur(ref other_contur) => {
                            inpolygon_switch(&mut inpolygon, circle_inside_contur(self_p, other_contur));

                            // If other contur lies inside self contur -> self contur is intersecting it
                            let mut other_inpolygon = self_contur.is_closed();

                            let other_points = &other_contur.points;

                            let mut other_first = other_points.first().unwrap();
                            for other_p in other_points {
                                let self_segment = (self_first, self_p);
                                let other_segment = (other_first, other_p);
                                inpolygon_switch(&mut other_inpolygon, circle_inside_contur(other_p, self_contur));
                                if lines_intersect(self_segment, other_segment) { return Some(Relation::Intersect) };
                                other_first = other_p;
                            };
                            if other_inpolygon { intersect = true };
                        },
                    }
                    self_first = self_p;
                };

                match inpolygon {
                    true => Some(Relation::Inside),
                    false => match intersect {
                        true => Some(Relation::Intersect),
                        false => None,
                    }
                }
            },
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
                },
                Some(Relation::Inside) => in_hole_counter += 1,
                None => continue,
            }
        };
        debug!("in hole_counter: {}", in_hole_counter);
        if in_hole_counter % 2 == 1 { return Some(Relation::Inside) };
        None
    }
}

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

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub r: Option<f64>,
}

impl Point {
    pub fn new(x: f64, y: f64, r: Option<f64>) -> Point {
        Point { x, y, r }
    }

    // should be reversed
    pub fn from_dxf_point(DxfPoint { x, y, .. }: &DxfPoint) -> Point {
        Point {
            x: *y,
            y: *x,
            r: None,
        }
    }

    pub fn from_dxf_circle(DxfCircle { center, radius, .. }: &DxfCircle) -> Point {
        let p = Point::from_dxf_point(&center);
        Point {
            r: Some(*radius),
            ..p
        }
    }

    pub fn is_circle(&self) -> bool {
        match self.r {
            Some(_) => true,
            None => false,
        }
    }

    pub fn is_point(&self) -> bool {
        match self.r {
            Some(_) => false,
            None => true,
        }
    }
}

impl Rectangable for Point {
    fn rect(&self) -> Rect {
        match self.r {
            Some(r) => Rect::from(self.x + r, self.y + r, self.x - r, self.y - r).unwrap(),
            None => Rect::from(self.x, self.y, self.x, self.y).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Contur {
    pub points: Vec<Point>,
}

impl Contur {
    pub fn new() -> Contur {
        Contur { points: vec![] }
    }
    pub fn push(&mut self, p: Point) {
        self.points.push(p)
    }
    pub fn is_closed(&self) -> bool {
        match self.points.last() {
            Some(l) => {
                if self.points[0] != *l {
                    return false;
                }
            }
            None => return false,
        };
        true
    }
    pub fn len(&self) -> usize {
        self.points.len()
    }
}

impl Rectangable for Contur {
    fn rect(&self) -> Rect {
        let mut rect = Rect::new();
        for p in &self.points {
            rect.add(p);
        };
        rect
    }
}

#[cfg(test)]
mod test;