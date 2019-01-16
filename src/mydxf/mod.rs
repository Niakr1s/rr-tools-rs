use dxf::entities::*;
use dxf::{Drawing, DxfResult};

pub struct MyDxf {
    path: String,
    drawing: Drawing,
}

impl MyDxf {
    pub fn from_file(path: &str) -> DxfResult<MyDxf> {
        let drawing = Drawing::load_file(path)?;
        let path = path.to_string();
        Ok(MyDxf { path, drawing })
    }
}

#[cfg(test)]
mod test;
