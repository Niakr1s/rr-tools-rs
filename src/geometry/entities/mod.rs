use dxf::entities::Circle as DxfCircle;
use dxf::Point as DxfPoint;

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub r: Option<f64>,
}

impl Point {
    pub fn new(x: f64, y: f64, r: Option<f64>) -> Point {
        Point { x, y, r }
    }

    // should be reversed
    pub fn from_dxf_point(DxfPoint { x, y, .. }: &DxfPoint) -> Point {
        Point {
            x: *y,
            y: *x,
            r: None,
        }
    }

    pub fn from_dxf_circle(DxfCircle { center, radius, .. }: &DxfCircle) -> Point {
        let p = Point::from_dxf_point(&center);
        Point {
            r: Some(*radius),
            ..p
        }
    }

    pub fn is_circle(&self) -> bool {
        match self.r {
            Some(_) => true,
            None => false,
        }
    }

    pub fn is_point(&self) -> bool {
        match self.r {
            Some(_) => false,
            None => true,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn from_points(start: Point, end: Point) -> Line {
        Line { start, end }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Contur {
    pub points: Vec<Point>,
}

impl Contur {
    pub fn new() -> Contur {
        Contur { points: vec![] }
    }
    pub fn add(&mut self, p: Point) {
        self.points.push(p)
    }
    pub fn is_closed(&self) -> bool {
        match self.points.last() {
            Some(l) => {
                if self.points[0] != *l {
                    return false;
                }
            }
            None => return false,
        };
        true
    }
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

#[cfg(test)]
mod test;