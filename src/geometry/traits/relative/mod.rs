use crate::geometry::entities::{Entities, Entity};
use crate::geometry::traits::rectangable::Rectangable;

pub trait Relative: Rectangable {
    fn relate_entity(&self, entity: &Entity) -> bool;
    fn relate_entities(&self, entities: &Entities) -> bool {
        for e in entities {
            if self.relate_entity(e) { return true }  // todo for "hole" entities
        };
        false
    }
}

//pub enum