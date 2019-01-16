use std::cmp::PartialEq;

use roxmltree::{self, Document};

use super::geometry::point::Point;
use super::scripts::*;

const CADASTRAL_NUMBER: &str = "CadastralNumber";

#[derive(Debug)]
pub struct RrXml {
    path: String,
    typ: String,
    number: String,
    parcels: Vec<Parcel>,
}

impl RrXml {
    pub fn from_file(path: &str) -> Result<RrXml, ()> {
        let file_content = match file_to_string(path) {
            Ok(fc) => fc,
            Err(_) => return Err(()),
        };
        let path = path.to_string();

        match RrXml::parse(&file_content) {
            Ok(rr_xml) => return Ok(RrXml { path, ..rr_xml }),
            Err(_) => return Err(()),
        }
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
                        let p = get_point_from_node(&p).unwrap(); //todo handle possible error
                        c.add(p);
                    }
                }
                let (typ, cad_number) = get_parent_type_and_number(&d);
                match parcels.iter_mut().find(|p| p.number == cad_number) {
                    Some(parcel) => {
                        trace!("'{} {}': adding contur: {:?}", parcel.typ, parcel.number, c);
                        parcel.add_contur(c);
                    }
                    None => {
                        let mut p = Parcel::new(typ, cad_number);
                        p.add_contur(c);
                        trace!(
                            "'{} {}': pushing with conturs: {:?}",
                            p.typ,
                            p.number,
                            p.conturs,
                        );
                        parcels.push(p);
                    }
                }
            }
        }

        let res = RrXml {
            path: String::new(),
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
pub struct Parcel {
    typ: String,
    number: String,
    conturs: Vec<Contur>,
}

impl Parcel {
    fn new(typ: String, number: String) -> Parcel {
        Parcel {
            typ: typ.to_string(),
            number: number.to_string(),
            conturs: vec![],
        }
    }
    fn add_contur(&mut self, c: Contur) {
        self.conturs.push(c);
    }
}

#[derive(Debug)]
pub struct Contur {
    pub points: Vec<Point>,
}

impl Contur {
    pub fn new() -> Contur {
        Contur { points: vec![] }
    }
    pub fn add(&mut self, p: Point) {
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
}

fn get_point_from_node(node: &roxmltree::Node<'_, '_>) -> Result<Point, ()> {
    let (x, y) = match (
        node.attribute("X").unwrap().parse::<f64>(),
        node.attribute("Y").unwrap().parse::<f64>(),
    ) {
        (Ok(x), Ok(y)) => (x, y),
        _ => return Err(()),
    };
    let mut r = None;
    for sibling in node.next_siblings() {
        if sibling.tag_name().name() == "R" {
            r = Some(sibling.text().unwrap().parse::<f64>().unwrap());
        }
    }
    let p = Point { x, y, r };
    Ok(p)
}

#[cfg(test)]
mod test;
