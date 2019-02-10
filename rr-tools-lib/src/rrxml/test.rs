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

    assert_eq!(
        point.unwrap(),
        Point::new(410328.96, 1230548.8, None)
    )
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
        Point::new(410328.96, 1230548.8, Some(0.5))
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
fn kpt_rect(){
    let rr = kpt();
    let rect = rr.rect();
    assert_eq!(rect, Rect::from(9233.9800, 24334.3300, 8652.3700, 22910.5700).unwrap());

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
    let new_filename = rr.new_filename();
    assert!(rr.new_filename().ends_with("KPT 77 03 0009007.xml"));

    let rr = kvzu();
    let new_filename = rr.new_filename();
    assert!(rr.new_filename().ends_with("KVZU 21 01 010206 115.xml"));
}