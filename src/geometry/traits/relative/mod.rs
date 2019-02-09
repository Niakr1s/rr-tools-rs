use crate::geometry::entities::*;
use crate::geometry::traits::rectangable::Rectangable;

pub trait Relative: Rectangable {
    fn relate_entity(&self, entity: &Entity) -> Option<Relation>;

    fn relate_entities(&self, entities: &Entities) -> Option<Relation> {
        let mut counter = 0;
        let mut in_hole: Option<bool> = None;
        println!("entities len: {}", entities.len());
        for e in entities {
            let relate = self.relate_entity(e);
            println!("got self.relate_entity {:?}", relate);
            match relate {
                Some(Relation::Intersect) => {
                    return Some(Relation::Intersect);
                },
                Some(Relation::Inside) => match in_hole {
                    Some(in_hole_bool) => {
                        println!("{:?}", e);
                        in_hole = Some(!in_hole_bool);
                        counter += 1;
                    },
                    None => in_hole = Some(false),
                },
                None => continue,
            }
        };
        println!("in hole: {:?} with counter: {}", in_hole, counter);
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