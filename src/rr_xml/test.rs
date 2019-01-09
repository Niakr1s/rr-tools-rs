use super::*;

const KVZU: &str = "KVZU Parcel 21 01 010206 115.xml";
const KPT: &str = "KPT CadastralBlock 77 03 0009007.xml";

const POINT_STR: &str = r#"<SpelementUnit TypeUnit="Точка" SuNmb="17">
                                <Ordinate X="410328.96" Y="1230548.8" OrdNmb="1" />
                            </SpelementUnit>"#;

const CIRCLE_STR: &str = r#"<SpelementUnit TypeUnit="Окружность" SuNmb="1">
                                    <Ordinate X="410328.96" Y="1230548.8" OrdNmb="1" DeltaGeopoint="0.1" />
                                    <R>0.5</R>
                                </SpelementUnit>"#;

#[test]
fn get_point_from_node_point() {
    let doc = Document::parse(POINT_STR).unwrap();
    println!("{:?}", doc);

    let mut point = None;

    for p in doc.descendants() {
        if p.tag_name().name() == "Ordinate" {
            point = Some(get_point_from_node(&p).unwrap());
        }
    }

    assert_eq!(
        point.unwrap(),
        Point {
            x: 410328.96,
            y: 1230548.8,
            r: 0.,
        }
    )
}

#[test]
fn get_point_from_node_circle() {
    let doc = Document::parse(CIRCLE_STR).unwrap();

    let mut point = None;

    for p in doc.descendants() {
        if p.tag_name().name() == "Ordinate" {
            point = Some(get_point_from_node(&p).unwrap());
        }
    }

    assert_eq!(
        point.unwrap(),
        Point {
            x: 410328.96,
            y: 1230548.8,
            r: 0.5,
        }
    )
}

#[test]
fn cadastral_number_is_true() {
    let rr = RrXml::from_file(KPT).unwrap();
    assert_eq!(rr.number, "77:03:0009007");
    assert_eq!(rr.typ, "KPT");
    let rr = RrXml::from_file(KVZU).unwrap();
    assert_eq!(rr.number, "21:01:010206:115");
    assert_eq!(rr.typ, "KVZU");
}

#[test]
fn point_is_circle_or_point() {
    let p1 = Point {
        x: 1.,
        y: 2.,
        r: 1.,
    };
    assert!(p1.is_circle());
    assert!(!p1.is_point());
    let p2 = Point {
        x: 1.,
        y: 2.,
        r: 0.,
    };
    assert!(p2.is_point());
    assert!(!p2.is_circle());
}

#[test]
fn point_partial_eq() {
    let p1 = Point {
        x: 1.,
        y: 1.,
        r: 1.,
    };
    let p2 = Point {
        x: 1.,
        y: 1.,
        r: 1.,
    };
    assert!(p1 == p2);
}
