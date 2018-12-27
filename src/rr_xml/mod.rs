#[cfg(test)]
mod test;

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
                        let p = get_point_from_node(&p);
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
    points: Vec<Point>,
}

impl Contur {
    fn new() -> Contur {
        Contur { points: vec![] }
    }
    fn add(&mut self, p: Point) {
        self.points.push(p)
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
    r: f64,
}

impl Point {
    fn is_circle(&self) -> bool {
        self.r != 0.
    }
    fn is_point(&self) -> bool {
        self.r == 0.
    }
}

fn get_point_from_node(node: &roxmltree::Node<'_, '_>) -> Point {
    let x = node.attribute("X").unwrap().parse::<f64>().unwrap();
    let y = node.attribute("Y").unwrap().parse::<f64>().unwrap();
    let mut r = 0.;
    for sibling in node.next_siblings() {
        if sibling.tag_name().name() == "R" {
            r = sibling.text().unwrap().parse::<f64>().unwrap();
        }
    }
    Point { x, y, r }
}
