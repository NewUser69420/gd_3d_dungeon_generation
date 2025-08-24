use std::collections::HashMap;

use fastrand::Rng;
use godot::builtin::{Vector3i};

pub struct Voxel {
    pub id: u8,
}

#[derive(Default, Clone, Copy)]
pub struct Aabb {
    pub min: Vector3i,
    pub max: Vector3i,
}

pub fn random_room() -> (HashMap<Vector3i, Voxel>, Aabb) {
    let mut rng = Rng::new();
    let mut voxels = HashMap::new();
    let mut aabb = Aabb::default();

    let xs = rng.i32(1..5);
    let ys = rng.i32(2..3);
    let zs = rng.i32(1..5);

    //get voxels and keep track of size
    for x in 0..xs {
        for y in 0..ys {
            for z in 0..zs {
                let pos = Vector3i { x, y, z };
                let voxel = Voxel { id: 0 };
                voxels.insert(pos, voxel);

                aabb.max = aabb.max.max(pos);
            }
        }
    }

    (voxels, aabb)
}