use rr_tools_rs::geometry::check_mydxf_in_rrxml;
use rr_tools_rs::mydxf;
use rr_tools_rs::rrxml;

#[test]
fn test_dxf() {
    let rr_block = rrxml::RrXml::from_file(r"src\test_files\xmls\KPT CadastralBlock 77 03 0009007.xml").unwrap();
    println!("{}", rr_block);

    let rr_parcel = rrxml::RrXml::from_file(r"src\test_files\xmls\KVZU Parcel 21 01 010206 115.xml").unwrap();
    println!("{}", rr_parcel);

    let mydxf = mydxf::MyDxf::from_file(r"src\test_files\mydxfs\6 1228.dxf").unwrap();
    println!("{:?}", mydxf);

    let res_block = check_mydxf_in_rrxml(&mydxf, &rr_block);
    println!("{:?}", res_block);

    let res_parcel = check_mydxf_in_rrxml(&mydxf, &rr_parcel);
    println!("{:?}", res_parcel);
}