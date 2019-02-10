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