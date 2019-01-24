use crate::geometry::entities::{Entities, Entity};
use crate::geometry::traits::rectangable::Rectangable;

pub trait Intersectable: Rectangable {
    fn intersect_entity(&self, entity: &Entity) -> bool;
    fn intersect_entities(&self, entities: &Entities) -> bool {
        for e in entities {
            if self.intersect_entity(e) { return true }  // todo for "hole" entities
        };
        false
    }
}
