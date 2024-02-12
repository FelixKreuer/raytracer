use super::vector3::{Vector3, Point3};

pub struct Ray {
    pub origin: Point3,
    pub dir: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vector3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f32) -> Vector3{
        return self.origin.clone() + (t * self.dir);
    }
}
