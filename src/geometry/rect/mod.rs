use crate::geometry::entities::*;

pub trait Rectangable {
    fn rect(&self) -> Rect;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Rect {
    pub xmax: f64,
    pub ymax: f64,
    pub xmin: f64,
    pub ymin: f64,
    empty: bool,
}

impl Rect {
    pub fn new() -> Rect {
        Rect {
            xmax: 0.,
            ymax: 0.,
            xmin: 0.,
            ymin: 0.,
            empty: true,
        }
    }

    pub fn from(xmax: f64, ymax: f64, xmin: f64, ymin: f64) -> Result<Rect, String> {
        let r = Rect { xmax, ymax, xmin, ymin, empty: false };
        if xmax < xmin || ymax < ymin {
            return Err(format!("rect {:?} is wrong, max value less then min", r))
        };
        Ok(Rect { xmax, ymax, xmin, ymin, empty: false })
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn add_rect(&mut self, other: &Rect) {
        if other.is_empty() { return };
        if self.is_empty() {
            *self = other.clone();
        }
        if other.xmax > self.xmax {
            self.xmax = other.xmax
        };
        if other.ymax > self.ymax {
            self.ymax = other.ymax
        };
        if other.xmin < self.xmin {
            self.xmin = other.xmin
        };
        if other.ymin < self.ymin {
            self.ymin = other.ymin
        };
    }

    /// returns true if self has changed
    pub fn add_point(&mut self, &Point {x, y, r}: &Point) {
        match r {
            Some(r) => {
                if self.is_empty() {
                    *self = Rect::from(x+r, y+r, x-r, y-r).unwrap();
                    return;
                };
                if x+r > self.xmax { self.xmax = x+r};
                if x-r < self.xmin { self.xmin = x-r};
                if y+r > self.ymax { self.ymax = y+r};
                if y-r < self.ymin { self.ymin = y-r};
            },
            None => {
                if self.is_empty() {
                    *self = Rect::from(x, y, x, y).unwrap();
                    return;
                };
                if x > self.xmax {
                    self.xmax = x
                } else if x < self.xmin {
                    self.xmin = x
                };
                if y > self.ymax {
                    self.ymax = y
                } else if y < self.ymin {
                    self.ymin = y
                };
            },
        }

    }
}
