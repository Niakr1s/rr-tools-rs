use std::cmp::PartialEq;

use roxmltree::{self, Document};

use super::scripts::*;

const CADASTRAL_NUMBER: &str = "CadastralNumber";

#[derive(Debug)]
pub struct RrXml {
    typ: String,
    number: String,
    parcels: Vec<Parcel>,
}

impl RrXml {
    pub fn from_file(filename: &str) -> Result<RrXml, ()> {
        let file_content = match file_to_string(filename) {
            Ok(fc) => fc,
            Err(_) => return Err(()),
        };

        match RrXml::from_str(&file_content) {
            Ok(rr_xml) => return Ok(rr_xml),
            Err(_) => return Err(()),
        }
    }

    pub fn from_str(input: &str) -> Result<RrXml, roxmltree::Error> {
        RrXml::parse(input)
    }

    fn parse(input: &str) -> Result<RrXml, roxmltree::Error> {
        let mut parcels: Vec<Parcel> = Vec::new();

        let root = Document::parse(input)?;

        let typ = root.root_element().tag_name().name().to_string();

        let number = root
            .descendants()
            .find(|d| d.has_attribute(CADASTRAL_NUMBER))
            .unwrap()
            .attribute(CADASTRAL_NUMBER)
            .unwrap()
            .to_string();

        for d in root.descendants() {
            if !d.is_element() {
                continue;
            };
            if d.tag_name().name() == "SpatialElement" {
                let mut c = Contur::new();
                for p in d.descendants() {
                    if p.tag_name().name() == "Ordinate" {
                        let p = get_geopoint_from_node(&p);
                        c.add(p);
                    }
                }
                let (typ, cad_number) = get_parent_type_and_number(&d);
                match parcels.iter_mut().find(|p| p.name == cad_number) {
                    Some(parcel) => {
                        trace!("'{} {}': adding contur: {:?}", parcel.typ, parcel.name, c);
                        parcel.add_contur(c);
                    }
                    None => {
                        let mut p = Parcel::new(typ, cad_number);
                        p.add_contur(c);
                        trace!(
                            "'{} {}': pushing with conturs: {:?}",
                            p.typ,
                            p.name,
                            p.conturs,
                        );
                        parcels.push(p);
                    }
                }
            }
        }

        let res = RrXml {
            typ,
            number,
            parcels,
        };

        info!("{:?}", res);

        Ok(res)
    }

    pub fn is_kpt(&self) -> bool {
        if self.number.split(":").collect::<Vec<&str>>().len() == 3 {
            return true;
        }
        false
    }
}

fn get_parent_type_and_number(node: &roxmltree::Node<'_, '_>) -> (String, String) {
    match node.attribute(CADASTRAL_NUMBER) {
        Some(attr) => (node.tag_name().name().to_string(), attr.to_string()),
        None => get_parent_type_and_number(&node.parent().unwrap()),
    }
}

#[derive(Debug)]
struct Parcel {
    typ: String,
    name: String,
    conturs: Vec<Contur>,
}

impl Parcel {
    fn new(typ: String, name: String) -> Parcel {
        Parcel {
            typ: typ.to_string(),
            name: name.to_string(),
            conturs: vec![],
        }
    }
    fn add_contur(&mut self, c: Contur) {
        self.conturs.push(c);
    }
}

#[derive(Debug)]
struct Contur {
    points: Vec<GeoPoint>,
}

impl Contur {
    fn new() -> Contur {
        Contur { points: vec![] }
    }
    fn add(&mut self, p: GeoPoint) {
        self.points.push(p)
    }
}

#[derive(Debug)]
enum GeoPoint {
    Point { x: f64, y: f64 },
    Circle { x: f64, y: f64, r: f64 },
}

fn get_geopoint_from_node(node: &roxmltree::Node<'_, '_>) -> GeoPoint {
    let x = node.attribute("X").unwrap().parse::<f64>().unwrap();
    let y = node.attribute("Y").unwrap().parse::<f64>().unwrap();
    for sibling in node.next_siblings() {
        if sibling.tag_name().name() == "R" {
            let r = sibling.text().unwrap().parse::<f64>().unwrap();
            return GeoPoint::Circle { x, y, r };
        }
    }
    GeoPoint::Point { x, y }
}

impl PartialEq for GeoPoint {
    fn eq(&self, other: &GeoPoint) -> bool {
        match (self, other) {
            (GeoPoint::Point { x: x1, y: y1 }, GeoPoint::Point { x: x2, y: y2 }) => {
                x1 == x2 && y1 == y2
            }
            (
                GeoPoint::Circle {
                    x: x1,
                    y: y1,
                    r: r1,
                },
                GeoPoint::Circle {
                    x: x2,
                    y: y2,
                    r: r2,
                },
            ) => x1 == x2 && y1 == y2 && r1 == r2,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test;
