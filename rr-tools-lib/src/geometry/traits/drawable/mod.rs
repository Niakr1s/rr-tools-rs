use dxf::{Drawing, DxfResult};

pub trait Drawable {
    fn draw(&self, drawing: &mut Drawing) -> DxfResult<()>;
}
