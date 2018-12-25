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
fn get_geopoint_from_node_point() {
    let doc = Document::parse(POINT_STR).unwrap();
    println!("{:?}", doc);

    let mut geopoint = None;

    for p in doc.descendants() {
        if p.tag_name().name() == "Ordinate" {
            geopoint = Some(get_geopoint_from_node(&p));
        }
    }

    assert_eq!(
        geopoint.unwrap(),
        GeoPoint::Point {
            x: 410328.96,
            y: 1230548.8
        }
    )
}

#[test]
fn get_geopoint_from_node_circle() {
    let doc = Document::parse(CIRCLE_STR).unwrap();

    let mut geopoint = None;

    for p in doc.descendants() {
        if p.tag_name().name() == "Ordinate" {
            geopoint = Some(get_geopoint_from_node(&p));
        }
    }

    assert_eq!(
        geopoint.unwrap(),
        GeoPoint::Circle {
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
