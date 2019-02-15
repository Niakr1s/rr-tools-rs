use crate::geometry::entities::*;
use std::ops::Add;

pub trait Rectangable {
    fn rect(&self) -> Rect;

    /// Negation of does_not_intersects method doesn't mean that objects intersect,
    fn can_not_intersect(&self, other: &impl Rectangable) -> bool {
        let (a, b) = (self.rect(), other.rect());
        a.xmax < b.xmin || a.ymax < b.ymin || a.xmin > b.xmax || a.ymin > b.ymax
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Rect {
    xmax: f64,
    ymax: f64,
    xmin: f64,
    ymin: f64,
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
        let r = Rect {
            xmax,
            ymax,
            xmin,
            ymin,
            empty: false,
        };
        if xmax < xmin || ymax < ymin {
            return Err(format!("rect {:?} is wrong, max value less then min", r));
        };
        Ok(Rect {
            xmax,
            ymax,
            xmin,
            ymin,
            empty: false,
        })
    }

    pub fn add(&mut self, other: &impl Rectangable) {
        let other = other.rect();
        self.add_rect(&other);
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    fn add_rect(&mut self, other: &Rect) {
        if other.is_empty() {
            return;
        };
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
}

impl Rectangable for Entities {
    fn rect(&self) -> Rect {
        let mut rect = Rect::new();
        for e in self {
            rect.add(e);
        }
        rect
    }
}

impl Rectangable for Entity {
    fn rect(&self) -> Rect {
        let mut rect = Rect::new();
        match self {
            Entity::Contur(ref c) => rect.add(c),
            Entity::Point(ref p) => rect.add(p),
        }
        rect
    }
}

impl Rectangable for Point {
    fn rect(&self) -> Rect {
        match self.r {
            Some(r) => Rect::from(self.x + r, self.y + r, self.x - r, self.y - r).unwrap(),
            None => Rect::from(self.x, self.y, self.x, self.y).unwrap(),
        }
    }
}

impl Rectangable for Contur {
    fn rect(&self) -> Rect {
        let mut rect = Rect::new();
        for p in &self.points {
            rect.add(p);
        }
        rect
    }
}
