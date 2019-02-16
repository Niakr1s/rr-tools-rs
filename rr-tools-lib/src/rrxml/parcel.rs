use crate::geometry::entities::*;
use crate::geometry::traits::rectangable::*;
// use roxmltree::{self, Document};

use dxf::entities as dxf_entities;
use dxf::entities::Entity as DxfEntity;
use dxf::Color;
use dxf::Point as DxfPoint;

const BLACK: u8 = 7;
const GREY: u8 = 8;
const GREEN: u8 = 63;

#[derive(Debug, Clone)]
pub struct Parcel {
    // may be: CadastralBlock, Parcel, Building, Construction, etc
    pub typ: String,
    pub number: String,
    pub entities: Entities,
}

impl Parcel {
    pub fn new(typ: String, number: String) -> Parcel {
        Parcel {
            typ: typ.to_string(),
            number: number.to_string(),
            entities: vec![],
        }
    }

    pub fn add_entity(&mut self, c: Entity) {
        self.entities.push(c);
    }

    fn to_dxf_entity_text(&self) -> DxfEntity {
        let (middle_x, middle_y) = self.get_middle_xy_inversed();
        let text = dxf_entities::Text {
            location: DxfPoint::new(middle_x, middle_y, 0.),
            value: self.dxf_string(),
            // horizontal_text_justification: HorizontalTextJustification::Center,
            ..Default::default()
        };
        let mut text_entity = dxf_entities::Entity::new(dxf_entities::EntityType::Text(text));
        text_entity.common.color = self.color();
        text_entity
    }

    // todo to dxf_block method, because couldn't find a way to properly creeate a block
    pub fn to_dxf_entities(&self) -> Vec<DxfEntity> {
        let mut dxf_entities = self
            .entities
            .iter()
            .map(|e| e.to_dxf_entity(self.color()))
            .collect::<Vec<DxfEntity>>();

        let text = self.to_dxf_entity_text();
        dxf_entities.push(text);

        for e in &mut dxf_entities {
            e.common.layer = self.dxf_string();
        }

        dxf_entities
    }

    fn color(&self) -> Color {
        match self.typ.as_ref() {
            "CadastralBlock" => Color::from_index(BLACK),
            "Parcel" => Color::from_index(GREY),
            _ => Color::from_index(GREEN),
        }
    }

    fn dxf_string(&self) -> String {
        format!("{} {}", self.number.replace(":", "_"), self.typ)
    }
}

impl Rectangable for Parcel {
    fn rect(&self) -> Rect {
        self.entities.rect()
    }
}
