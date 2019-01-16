use super::*;

#[test]
fn triangle_polyline_ok() {
    let path = "src/mydxf/test/dxfs/triangle_polyline.dxf";
    let my_dxf = MyDxf::from_file(path);
    if let Ok(my_dxf) = my_dxf.as_ref() {
        for e in &my_dxf.drawing.entities {
            println!("{:?}", e.specific);
        }
    }
    assert!(my_dxf.is_ok(), "test file open error");
}

#[test]
fn triangle_line_ok() {
    let path = "src/mydxf/test/dxfs/triangle_line.dxf";
    let my_dxf = MyDxf::from_file(path);
    if let Ok(my_dxf) = my_dxf.as_ref() {
        for e in &my_dxf.drawing.entities {
            println!("{:?}", e.specific);
        }
    }
    assert!(my_dxf.is_ok(), "test file open error");
}

#[test]
fn circle_ok() {
    let path = "src/mydxf/test/dxfs/circle.dxf";
    let my_dxf = MyDxf::from_file(path);
    if let Ok(my_dxf) = my_dxf.as_ref() {
        for e in &my_dxf.drawing.entities {
            println!("{:?}", e.specific);
        }
    }
    assert!(my_dxf.is_ok(), "test file open error");
}

#[test]
fn point_ok() {
    let path = "src/mydxf/test/dxfs/point.dxf";
    let my_dxf = MyDxf::from_file(path);
    if let Ok(my_dxf) = my_dxf.as_ref() {
        for e in &my_dxf.drawing.entities {
            println!("{:?}", e.specific);
        }
    }
    assert!(my_dxf.is_ok(), "test file open error");
}
