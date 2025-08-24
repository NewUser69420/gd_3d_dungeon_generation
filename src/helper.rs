use fastrand::Rng;
use godot::builtin::Vector3i;

use crate::voxel::Aabb;

pub fn random_vec3(min: i32, max: i32) -> Vector3i {
    let mut rng = Rng::new();

    let x = rng.i32(min..max);
    let y = rng.i32(min..max);
    let z = rng.i32(min..max);

    Vector3i {x, y, z}
}

pub fn aabb_coll(a: Aabb, b: Aabb) -> bool {
    a.min.x <= b.max.x
	    && a.max.x >= b.min.x
	    && a.min.y <= b.max.y
	    && a.max.y >= b.min.y
}