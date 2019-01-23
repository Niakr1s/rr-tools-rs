use std::cmp::PartialEq;

use roxmltree::{self, Document};

use super::geometry::entities::*;
use std::fs::File;
use std::io;
use std::io::Read;
use std::error::Error;
use crate::error::MyError;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use crate::Rectangable;

const CADASTRAL_NUMBER: &str = "CadastralNumber";

#[derive(Debug)]
pub struct RrXml {
    pub path: String,
    pub typ: String,
    pub number: String,
    pub parcels: Vec<Parcel>,
}

impl RrXml {
    pub fn from_file(path: &str) -> Result<RrXml, Box<dyn Error>> {
        let file_content = file_to_string(path)?;
        let path = path.to_string();

        let parsed = RrXml::parse(&file_content)?;
        Ok (RrXml { path, ..parsed})
    }

    fn parse(input: &str) -> Result<RrXml, Box<dyn Error>> {
        let mut parcels: Vec<Parcel> = Vec::new();

        let root = Document::parse(input)?;

        let typ = root.root_element().tag_name().name().to_string();

        let number = root
            .descendants()
            .find(|d| d.has_attribute(CADASTRAL_NUMBER))
            .expect(r#"no attribute "Cadastral Number"#)
            .attribute(CADASTRAL_NUMBER)
            .expect(r#"no attribute "Cadastral Number"#)
            .to_string();

        for d in root.descendants() {
            if !d.is_element() {
                continue;
            };
            if d.tag_name().name() == "SpatialElement" {
                let mut c = Contur::new();
                for p in d.descendants() {
                    if p.tag_name().name() == "Ordinate" {
                        let p = get_point_from_node(&p)?;
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

        debug!("{:?}", res);



        Ok(res)
    }

    pub fn is_kpt(&self) -> bool {
        if self.number.split(":").collect::<Vec<&str>>().len() == 3 {
            return true;
        }
        false
    }

    /// It's simple, kpt is usually the last in xml file
    pub fn get_kpt_parcel(&self) -> Option<&Parcel> {
        if !self.is_kpt() { return None };
        self.parcels.last()
    }

    pub fn len(&self) -> usize {
        self.parcels.len()
    }

}

impl Rectangable for RrXml {
    fn rect(&self) -> Rect {
        unimplemented!()
    }
}

impl Display for RrXml {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Got {typ}:{number} from '{path}'",
            path = self.path, typ = self.typ, number = self.number,
        )?;
        writeln!(f, "with parcels:")?;
        for p in &self.parcels {
            writeln!(f, "\t{}", p.number)?;
        };
        writeln!(f, "")
    }
}

fn get_parent_type_and_number(node: &roxmltree::Node<'_, '_>) -> (String, String) {
    match node.attribute(CADASTRAL_NUMBER) {
        Some(attr) => (node.tag_name().name().to_string(), attr.to_string()),
        None => get_parent_type_and_number(&node.parent().unwrap()),
    }
}

#[derive(Debug, Clone)]
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

impl Rectangable for Parcel {
    fn rect(&self) -> Rect {
        let mut rect = Rect::new();
        for c in &self.conturs {
            rect.add_rect(&c.rect());
        };
        rect
    }
}

fn get_point_from_node(node: &roxmltree::Node<'_, '_>) -> Result<Point, Box<dyn Error>> {
    let (x, y) = match (
        node.attribute("X").unwrap().parse::<f64>(),
        node.attribute("Y").unwrap().parse::<f64>(),
    ) {
        (Ok(x), Ok(y)) => (x, y),
        _ => return Err(MyError::new(format!("couldn't get X and Y from {:?}", node)).into()),
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

fn file_to_string(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut f = File::open(filename)?;
    let mut file_content = String::new();
    f.read_to_string(&mut file_content)?;
    Ok(file_content)
}


#[cfg(test)]
mod test;
