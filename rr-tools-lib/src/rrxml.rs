use super::geometry::entities::*;
use crate::error::MyError;
use crate::geometry::traits::rectangable::*;
use roxmltree::{self, Document};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use dxf::entities::Entity as DxfEntity;
use dxf::{Drawing, DxfResult};

pub mod parcel;
use parcel::Parcel;

const CADASTRAL_NUMBER: &str = "CadastralNumber";

#[derive(Debug, Clone)]
pub struct RrXml {
    pub path: String,
    pub typ: String,
    pub number: String,
    pub parcels: Vec<Parcel>,
}

impl RrXml {
    pub fn from_file(path: &str) -> Result<RrXml, Box<dyn Error>> {
        debug!("attempt to parse xml: {}", path);
        let file_content = file_to_string(path)?;
        let path = path.to_string();

        let parsed = RrXml::parse(&file_content)?;
        debug!("succesfully parsed xml: {}", parsed);
        Ok(RrXml { path, ..parsed })
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
                        let p = point_from_node_chunk(&p)?;
                        c.push(p);
                    }
                }
                let c = match Entity::from_contur(c) {
                    Some(e) => e,
                    None => continue,
                };
                let (typ, cad_number) = get_parent_type_and_number(&d);
                match parcels.iter_mut().find(|p| p.number == cad_number) {
                    Some(parcel) => {
                        trace!("'{} {}': adding contur: {:?}", parcel.typ, parcel.number, c);
                        parcel.add_entity(c);
                    }
                    None => {
                        let mut p = Parcel::new(typ, cad_number);
                        p.add_entity(c);
                        trace!(
                            "'{} {}': pushing with conturs: {:?}",
                            p.typ,
                            p.number,
                            p.entities,
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
        if self.number.split(':').count() == 3 {
            return true;
        }
        false
    }

    /// It's simple, kpt is usually the last in xml file
    pub fn get_kpt_parcel(&self) -> Option<&Parcel> {
        if !self.is_kpt() {
            return None;
        };
        self.parcels.last()
    }

    pub fn len(&self) -> usize {
        self.parcels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.parcels.is_empty()
    }

    pub fn rename_file(&self) -> io::Result<String> {
        let new_filepath = self.new_filepath();
        if self.new_filepath() == self.path {
            info!("no need to rename: {}", self.path);
            return Ok(new_filepath); // here new_filepath == self.path, so we can move out
        }
        info!("trying to rename: {}", self.path);
        fs::rename(&self.path, &new_filepath)?;
        info!("succesfully renamed to: {}", new_filepath);
        Ok(new_filepath)
    }

    pub fn new_filepath(&self) -> String {
        let path = Path::new(&self.path);
        let new_filename = format!("{} {}", self.typ, self.number.replace(":", " "));
        let mut new_path = path.with_file_name(new_filename);
        if let Some(ext) = path.extension() {
            new_path.set_extension(ext);
        }
        debug!("rrxml old path: {:?}, new path: {:?}", path, new_path);
        new_path.to_str().unwrap().to_string()
    }

    pub fn save_to_dxf(&self) -> DxfResult<()> {
        let mut path = PathBuf::from(&self.path);
        path.set_extension("dxf");
        let path = path.to_str().unwrap();
        self.to_drawing().save_file(path)
    }

    pub fn to_entities(&self) -> Vec<DxfEntity> {
        let mut entities: Vec<DxfEntity> = vec![];
        for p in &self.parcels {
            let mut parcel_entities = p.to_dxf_entities();
            entities.append(&mut parcel_entities);
        }
        entities
    }

    fn to_drawing(&self) -> Drawing {
        Drawing {
            entities: self.to_entities(),
            ..Default::default()
        }
    }
}

impl Rectangable for RrXml {
    fn rect(&self) -> Rect {
        let mut rect = Rect::new();
        for p in &self.parcels {
            rect.add(p)
        }
        rect
    }
}

impl Display for RrXml {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "RrXml {typ} {number} from {path}",
            path = self.path,
            typ = self.typ,
            number = self.number,
        )?;
        writeln!(f, "with parcels:")?;
        for p in &self.parcels {
            writeln!(f, "\t{}", p.number)?;
        }
        writeln!(f)
    }
}

fn file_to_string(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut f = File::open(filename)?;
    let mut file_content = String::new();
    f.read_to_string(&mut file_content)?;
    Ok(file_content)
}

fn get_parent_type_and_number(node: &roxmltree::Node<'_, '_>) -> (String, String) {
    match node.attribute(CADASTRAL_NUMBER) {
        Some(attr) => (node.tag_name().name().to_string(), attr.to_string()),
        None => get_parent_type_and_number(&node.parent().unwrap()),
    }
}

fn point_from_node_chunk(node: &roxmltree::Node<'_, '_>) -> Result<Point, Box<dyn Error>> {
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

#[cfg(test)]
mod test {
    use super::*;

    const KPT: &str = r"src\test_files\xmls\KPT CadastralBlock 77 03 0009007.xml";
    const KVZU: &str = r"src\test_files\xmls\KVZU Parcel 21 01 010206 115.xml";

    const POINT_STR: &str = r#"<SpelementUnit TypeUnit="Точка" SuNmb="17">
                                    <Ordinate X="410328.96" Y="1230548.8" OrdNmb="1" />
                                </SpelementUnit>"#;

    const CIRCLE_STR: &str = r#"<SpelementUnit TypeUnit="Окружность" SuNmb="1">
                                        <Ordinate X="410328.96" Y="1230548.8" OrdNmb="1" DeltaGeopoint="0.1" />
                                        <R>0.5</R>
                                    </SpelementUnit>"#;

    fn kpt() -> RrXml {
        RrXml::from_file(KPT).unwrap()
    }

    fn kvzu() -> RrXml {
        RrXml::from_file(KVZU).unwrap()
    }

    #[test]
    fn get_point_from_node_point() {
        let doc = Document::parse(POINT_STR).unwrap();

        let mut point = None;

        for p in doc.descendants() {
            if p.tag_name().name() == "Ordinate" {
                point = Some(point_from_node_chunk(&p).unwrap());
            }
        }

        assert_eq!(point.unwrap(), Point::new(410_328.96, 1_230_548.8, None))
    }

    #[test]
    fn get_point_from_node_circle() {
        let doc = Document::parse(CIRCLE_STR).unwrap();

        let mut point = None;

        for p in doc.descendants() {
            if p.tag_name().name() == "Ordinate" {
                point = Some(point_from_node_chunk(&p).unwrap());
            }
        }

        assert_eq!(
            point.unwrap(),
            Point::new(410_328.96, 1_230_548.8, Some(0.5))
        )
    }

    #[test]
    fn cadastral_number_is_true() {
        let rr = kpt();
        assert_eq!(rr.number, "77:03:0009007");
        assert_eq!(rr.typ, "KPT");
        let rr = kvzu();
        assert_eq!(rr.number, "21:01:010206:115");
        assert_eq!(rr.typ, "KVZU");
    }

    #[test]
    fn get_kpt_parcel_ok() {
        let rr = kpt();
        assert!(rr.is_kpt());
        let p = rr.get_kpt_parcel();
        match p {
            Some(p) => assert_eq!(p.number, "77:03:0009007"),
            None => panic!("xml is not empty!"),
        }
    }

    #[test]
    fn kpt_rect() {
        let rr = kpt();
        let rect = rr.rect();
        assert_eq!(
            rect,
            Rect::from(9233.9800, 24334.3300, 8652.3700, 22910.5700).unwrap()
        );
    }

    #[test]
    fn point_is_circle_or_point() {
        let p1 = Point::new(1., 2., Some(1.));
        assert!(p1.is_circle());
        assert!(!p1.is_point());
        let p2 = Point::new(1., 2., None);
        assert!(p2.is_point());
        assert!(!p2.is_circle());
    }

    #[test]
    fn point_partial_eq() {
        let p1 = Point::new(1., 1., Some(1.));
        let p2 = Point::new(1., 1., Some(1.));
        assert_eq!(p1, p2);
    }

    #[test]
    fn new_filename() {
        let rr = kpt();
        assert!(rr.new_filepath().ends_with("KPT 77 03 0009007.xml"));

        let rr = kvzu();
        assert!(rr.new_filepath().ends_with("KVZU 21 01 010206 115.xml"));
    }

    #[test]
    fn rrxml_save_to_dxf() {
        let rr = kpt();
        assert!(rr.save_to_dxf().is_ok());
    }

}
