use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use roxmltree::Document;

const CADASTRAL_NUMBER: &str = "CadastralNumber";

#[derive(Debug)]
pub struct RrXml {
    t: String,
    is_kpt: bool,
    number: String,
    parcels: Vec<Parcel>,
}

impl RrXml {
    pub fn from_file(filename: &str) -> Result<RrXml, ()> {
        let mut f = File::open(filename).unwrap(); // todo
        let mut file_content = String::new();
        f.read_to_string(&mut file_content);

        RrXml::parse(&file_content)
    }

    pub fn from_str(input: &str) -> Result<RrXml, ()> {
        RrXml::parse(input)
    }

    fn parse(input: &str) -> Result<RrXml, ()> {
        let number = String::new();
        let mut parcels: Vec<Parcel> = Vec::new();

        let root = Document::parse(input).unwrap();

        let t = root.root_element().tag_name().name().to_string();

        let number = root
            .descendants()
            .find(|d| d.has_attribute(CADASTRAL_NUMBER))
            .unwrap()
            .attribute(CADASTRAL_NUMBER)
            .unwrap()
            .to_string();
        let is_kpt = is_kpt(&number);

        for d in root.descendants() {
            if !d.is_element() {
                continue;
            };
            if d.tag_name().name() == "SpatialElement" {
                let mut c = Contur::new();
                for p in d.descendants() {
                    if p.tag_name().name() == "Ordinate" {
                        let x = p.attribute("X").unwrap().parse::<f64>().unwrap();
                        let y = p.attribute("Y").unwrap().parse::<f64>().unwrap();
                        let p = Point::new(x, y);
                        c.add(p);
                    }
                }
                let cad_number = get_parent_cadastral_number(d);
                match parcels.iter_mut().find(|p| p.name == cad_number) {
                    Some(parcel) => {
                        info!("{:?}: adding contur: {:?}", parcel.name, c);
                        parcel.add_contur(c);
                    }
                    None => {
                        let mut p = Parcel::new(&cad_number);
                        p.add_contur(c);
                        info!("{:?}: pushing with conturs: {:?}", p.name, p.conturs,);
                        parcels.push(p);
                    }
                }
            }
        }

        Ok(RrXml {
            t,
            is_kpt,
            number,
            parcels,
        })
    }
}

fn get_parent_cadastral_number(node: roxmltree::Node<'_, '_>) -> String {
    match node.attribute(CADASTRAL_NUMBER) {
        Some(attr) => attr.to_string(),
        None => get_parent_cadastral_number(node.parent().unwrap()),
    }
}

fn rr_xml_name_from_str(n: String) -> Name {
    let l = n.split(":").collect::<Vec<&str>>().len();
    if l == 3 {
        return Name::Kpt(n);
    }
    Name::NonKpt(n)
}

#[derive(Debug)]
enum Name {
    Kpt(String),
    NonKpt(String),
}

#[derive(Debug)]
struct Parcel {
    name: String,
    conturs: Vec<Contur>,
}

impl Parcel {
    fn new(name: &str) -> Parcel {
        Parcel {
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

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

fn is_kpt(s: &str) -> bool {
    if s.split(":").collect::<Vec<&str>>().len() == 3 {
        return true;
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    const KVZU: &str = "KVZU Parcel 21 01 010206 115.xml";
    const KPT: &str = "KPT CadastralBlock 77 03 0009007.xml";

    #[test]
    fn cadastral_number_is_true() {
        let rr = RrXml::from_file(KPT).unwrap();
        assert_eq!(rr.number, "77:03:0009007");
        let rr = RrXml::from_file(KVZU).unwrap();
        assert_eq!(rr.number, "21:01:010206:115");
    }
}
