pub struct Ray {
    pub root: Vector3,
    pub dir: Vector3
}

impl Ray {
    pub fn new(root: Point3, dir: Vector3) -> Ray{
        Ray{root, dir}
    }

    pub fn at(&self, t:f32){
        return root + t*dir;
    }
}