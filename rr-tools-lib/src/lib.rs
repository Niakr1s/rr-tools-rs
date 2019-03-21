#[macro_use]
extern crate log;

#[macro_use]
pub mod geometry;
pub mod error;
pub mod mydxf;
pub mod rrxml;

use crate::geometry::traits::relative::Relative;
use crate::mydxf::MyDxf;
use crate::rrxml::parcel::Parcel;
use crate::rrxml::RrXml;

pub fn check_mydxf_in_rrxmls(mydxf: &MyDxf, rrxmls: Vec<RrXml>) -> Option<Vec<Parcel>> {
    let mut parcels = vec![];
    for rrxml in rrxmls {
        if let Some(ref mut parcel) = check_mydxf_in_rrxml(mydxf, &rrxml) {
            parcels.append(parcel);
        }
    }
    match parcels.len() {
        0 => None,
        _ => Some(parcels),
    }
}

pub fn check_mydxf_in_rrxml(mydxf: &MyDxf, rrxml: &RrXml) -> Option<Vec<Parcel>> {
    if rrxml.len() == 0 {
        return None;
    };

    let mut parcels = vec![];

    for parcel in &rrxml.parcels {
        if check_mydxf_in_parcel(&mydxf, &parcel) {
            parcels.push(parcel.clone());
        }
    }
    match parcels.len() {
        0 => None,
        _ => Some(parcels),
    }
}

pub fn check_mydxf_in_parcel(mydxf: &MyDxf, parcel: &Parcel) -> bool {
    match mydxf.entities.relate_entities(&parcel.entities) {
        Some(_) => {
            debug!("got intersect with {}", parcel.number);
            true
        }
        None => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mydxf::MyDxf;
    use crate::rrxml::RrXml;
    use std::path::Path;

    #[test]
    fn test_dxf() {
        let rr_block =
            RrXml::from_file(r"src\test_files\xmls\KPT CadastralBlock 77 03 0009007.xml").unwrap();
        println!("{}", rr_block);

        let rr_parcel =
            RrXml::from_file(r"src\test_files\xmls\KVZU Parcel 21 01 010206 115.xml").unwrap();
        println!("{}", rr_parcel);

        let mydxf = MyDxf::from_file(r"src\test_files\mydxfs\6 1228.dxf").unwrap();
        println!("{:?}", mydxf);

        let res_block = check_mydxf_in_rrxml(&mydxf, &rr_block).unwrap();
        assert_eq!(res_block.len(), 4);

        let res_parcel = check_mydxf_in_rrxml(&mydxf, &rr_parcel);
        assert!(res_parcel.is_none());
    }

    #[test]
    fn test_from_rrtools_python() {
        let xml_dir = Path::new(r"src\test_files\tests_from_rrtools_python\xml");
        let mut rrxmls = vec![];
        for f in xml_dir.read_dir().expect("dir call error") {
            if let Ok(f) = f {
                if let Some(ext) = f.path().extension() {
                    if ext == "xml" {
                        let rrxml = RrXml::from_file(f.path().to_str().unwrap()).unwrap();
                        rrxmls.push(rrxml);
                    }
                }
            }
        }

        let mydxf = MyDxf::from_file(&dxf_path(1)).unwrap();
        let res = check_mydxf_in_rrxmls(&mydxf, rrxmls.clone()).unwrap();
        assert_eq!(res.len(), 4);

        let mydxf = MyDxf::from_file(&dxf_path(2)).unwrap();
        let res = check_mydxf_in_rrxmls(&mydxf, rrxmls.clone()).unwrap();
        assert_eq!(res.len(), 3);

        let mydxf = MyDxf::from_file(&dxf_path(3)).unwrap();
        let res = check_mydxf_in_rrxmls(&mydxf, rrxmls.clone()).unwrap();
        assert_eq!(res.len(), 2);

        let mydxf = MyDxf::from_file(&dxf_path(4)).unwrap();
        let res = check_mydxf_in_rrxmls(&mydxf, rrxmls.clone()).unwrap();
        assert_eq!(res.len(), 2);

        let mydxf = MyDxf::from_file(&dxf_path(5)).unwrap();
        let res = check_mydxf_in_rrxmls(&mydxf, rrxmls.clone()).unwrap();
        assert_eq!(res.len(), 3);

        let mydxf = MyDxf::from_file(&dxf_path(6)).unwrap();
        let res = check_mydxf_in_rrxmls(&mydxf, rrxmls.clone());
        assert!(res.is_none());

        fn dxf_path(i: i32) -> String {
            format!(
                r"src\test_files\tests_from_rrtools_python\mydxf\test {}.dxf",
                i
            )
        }
    }

}
