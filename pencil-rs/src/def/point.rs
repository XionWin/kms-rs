use crate::def::as_primitive::AsPrimitive;

#[derive(Debug, Copy, Clone, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    pub fn equals(self, pt: Point, tol: f32) -> bool {
        let dx = pt.x - self.x;
        let dy = pt.y - self.y;
        dx * dx + dy * dy < tol * tol
    }

    pub fn dist_pt_seg(self, p: Point, q: Point) -> f32 {
        let pqx = q.x - p.x;
        let pqy = q.y - p.y;
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        let d = pqx * pqx + pqy * pqy;
        let mut t = pqx * dx + pqy * dy;
        if d > 0.0 {
            t /= d;
        }
        if t < 0.0 {
            t = 0.0
        } else if t > 1.0 {
            t = 1.0
        };
        let dx = p.x + t * pqx - self.x;
        let dy = p.y + t * pqy - self.y;
        dx * dx + dy * dy
    }

    pub fn normalize(&mut self) -> f32 {
        let d = ((self.x) * (self.x) + (self.y) * (self.y)).sqrt();
        if d > 1e-6 {
            let id = 1.0 / d;
            self.x *= id;
            self.y *= id;
        }
        d
    }

    pub fn cross(pt1: Point, pt2: Point) -> f32 {
        pt2.x * pt1.y - pt1.x * pt2.y
    }

    pub fn offset(&self, tx: f32, ty: f32) -> Point {
        Point::new(self.x + tx, self.y + ty)
    }
}

impl<T: AsPrimitive<f32>> From<(T, T)> for Point {
    fn from((x, y): (T, T)) -> Self {
        Point::new(x.as_(), y.as_())
    }
}