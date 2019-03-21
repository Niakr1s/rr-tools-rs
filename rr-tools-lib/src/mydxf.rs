use crate::geometry::entities::*;
use crate::geometry::traits::rectangable::*;
use dxf::entities::EntityType;
use dxf::{Drawing, DxfResult};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct MyDxf {
    pub path: String,
    pub entities: Entities,
}

impl MyDxf {
    pub fn from_file(path: &str) -> DxfResult<MyDxf> {
        debug!("attempt to parse dxf: {}", path);
        let path = path.to_string();
        let drawing = Drawing::load_file(&path)?;
        let entities = drawing_to_entities(drawing);
        let parsed = MyDxf { path, entities };
        debug!("succesfully parsed dxf: {}", parsed);

        Ok(parsed)
    }
}

impl Display for MyDxf {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "MyDxf from {path}", path = self.path,)?;
        writeln!(f, "with {} entities", self.entities.len())?;
        writeln!(f)
    }
}

impl Rectangable for MyDxf {
    fn rect(&self) -> Rect {
        self.entities.rect()
    }
}

fn drawing_to_entities(drawing: Drawing) -> Vec<Entity> {
    let mut entities = vec![];
    for e in drawing.entities {
        let contur = match e.specific {
            EntityType::LwPolyline(lw_polyline) => {
                let mut contur = Contur::new();
                for p in lw_polyline.vertices {
                    contur.push(Point::new(p.y, p.x, None)); // should be reversed
                }
                Entity::Contur(contur)
            }
            EntityType::Polyline(polyline) => {
                let mut contur = Contur::new();
                for p in polyline.vertices {
                    contur.push(Point::from_dxf_point(&p.location));
                }
                Entity::Contur(contur)
            }
            EntityType::Line(line) => {
                let p1 = Point::from_dxf_point(&line.p1);
                let p2 = Point::from_dxf_point(&line.p2);
                Entity::Contur(contur![p1, p2])
            }
            EntityType::Circle(circle) => Entity::Point(Point::from_dxf_circle(&circle)),
            EntityType::ModelPoint(model_point) => {
                Entity::Point(Point::from_dxf_point(&model_point.location))
            }
            _ => continue,
        };
        entities.push(contur);
    }
    entities
}

#[cfg(test)]
mod test {
    use super::*;

    /// In this test module result should be with reversed coordinates

    const P1: Point = Point {
        x: 1.5,
        y: 1.5,
        r: None,
    };
    const P2: Point = Point {
        x: 2.5,
        y: 2.5,
        r: None,
    };
    const P3: Point = Point {
        x: -3.0,
        y: 2.0,
        r: None,
    };

    #[test]
    fn triangle_polyline() {
        let path = r"src\test_files\dxfs\triangle_polyline.dxf";
        let my_dxf = MyDxf::from_file(path);
        assert!(my_dxf.is_ok(), "test file {} open error", path);
        let my_dxf = my_dxf.unwrap();

        let contur = contur![P1.clone(), P2.clone(), P3.clone(), P1.clone()];
        assert_eq!(my_dxf.entities, vec![Entity::Contur(contur),]);
    }

    #[test]
    fn triangle_line() {
        let path = r"src\test_files\dxfs\triangle_line.dxf";
        let my_dxf = MyDxf::from_file(path);
        assert!(my_dxf.is_ok(), "test file {} open error", path);
        let my_dxf = my_dxf.unwrap();

        let contur1 = contur![P1.clone(), P2.clone()];
        let contur2 = contur![P2.clone(), P3.clone()];
        let contur3 = contur![P3.clone(), P1.clone()];

        assert_eq!(
            my_dxf.entities,
            vec![
                Entity::Contur(contur1),
                Entity::Contur(contur2),
                Entity::Contur(contur3),
            ]
        );
    }

    #[test]
    fn circle() {
        let path = r"src\test_files\dxfs\circle.dxf";
        let my_dxf = MyDxf::from_file(path);
        assert!(my_dxf.is_ok(), "test file {} open error", path);
        let my_dxf = my_dxf.unwrap();

        assert_eq!(
            my_dxf.entities,
            vec![Entity::Point(Point::new(1.5, 1.5, Some(1.5)))]
        );
    }

    #[test]
    fn point() {
        let path = r"src\test_files\dxfs\point.dxf";
        let my_dxf = MyDxf::from_file(path);
        assert!(my_dxf.is_ok(), "test file {} open error", path);
        let my_dxf = my_dxf.unwrap();

        assert_eq!(my_dxf.entities, vec![Entity::Point(P1.clone())]);
    }

}
