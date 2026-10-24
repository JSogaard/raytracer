use std::assert_eq;

use raytracer::vec3::Vec3;

fn main() {
    let a = Vec3::new(3., 1., 4.);
    let b = Vec3::new(-2., 2., -3.);

    assert_eq!(a.dot(b), -16.);
    assert_eq!(a.cross(b), Vec3::new(-11., 1., 8.));
}
