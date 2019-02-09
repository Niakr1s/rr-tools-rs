use crate::geometry::entities::*;
use crate::geometry::traits::rectangable::Rectangable;

pub trait Relative: Rectangable {
    fn relate_entity(&self, entity: &Entity) -> Option<Relation>;

    fn relate_entities(&self, entities: &Entities) -> Option<Relation> {
        let mut in_hole: Option<bool> = None;
        for e in entities {
            let relate = self.relate_entity(e);
            println!("got self.relate_entity {:?}", relate);
            match relate {
                Some(Relation::Intersect) => {
                    return Some(Relation::Intersect);
                },
                Some(Relation::Inside) => match in_hole {
                    Some(in_hole_bool) => in_hole = Some(!in_hole_bool),
                    None => in_hole = Some(false),
                },
                None => continue,
            }
        };
        println!("in hole: {:?}", in_hole);
        match in_hole {
            Some(true) => None,
            Some(false) => Some(Relation::Inside),
            None => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Relation {
    Inside,
    Intersect,
}