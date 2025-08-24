// voxel based, 3d room generator.
// 
// must have interesting 3d shapes, and a 3d layout,
// with corridors connecting the rooms.
//
// rooms are represented by the gridmap in godot,
// using cut up 3d tiles.
//
// also needs to be fast.

use std::collections::HashMap;
use godot::prelude::*;

use crate::{helper::{aabb_coll, random_vec3}, voxel::{random_room, Aabb, Voxel}};

const EXPLODE_SPEED: f32 = 2.5;

struct Room {
    position: Vector3i,
    voxels: HashMap<Vector3i, Voxel>,
    aabb: Aabb,
}
impl Room {
    fn apply_dir(&mut self, dir: Vector3, speed: f32) {
        let position = self.position.cast_float() + (dir * speed);
        self.position = position.cast_int();
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Dungeon {
    rooms: Vec<Room>,
    corridors: Vec<Room>,

    base: Base<Node>,
}
#[godot_api]
impl INode for Dungeon {
    fn init(base: Base<Node>) -> Self {
        Dungeon { 
            rooms: Vec::new(), 
            corridors: Vec::new(),
            base,
        }
    }
}

#[godot_api]
impl Dungeon {
    #[func]
    fn generate(&mut self, room_count: i32) {
        //gen rooms
        let mut rms = gen_rooms(room_count);
        //explode rooms
        if !rms.is_empty() { explode_rooms(&mut rms); }
        //gen corridors
        let mut cordrs = Vec::new();

        self.rooms = rms;
        self.corridors = cordrs;
    }

    #[func]
    fn get_tiles(&self) -> Dictionary {
        let mut dic = Dictionary::new();
        
        for room in self.rooms.iter() {
            for (pos, vox) in room.voxels.iter() {
                let position = room.position + *pos;
                dic.set(position, vox.id);
            }
        }

        for corr in self.corridors.iter() {
            for (pos, vox) in corr.voxels.iter() {
                let position = corr.position + *pos;
                dic.set(position, vox.id);
            }
        }

        dic
    }
}

fn gen_rooms(rm_count: i32) -> Vec<Room> {
    let mut rooms = Vec::new();

    for _ in 0..rm_count {
        //generate room shape
        let position = random_vec3(-5, 5);
        let (voxels, aabb) = random_room();

        rooms.push(Room {
            position,
            voxels,
            aabb,
        });
    }

    rooms
}

#[derive(Default)]
struct Intersection {
    id: usize,
    dir: Vector3,
}

fn explode_rooms(rooms: &mut Vec<Room>) {
    let rm_len = rooms.len();

    let mut intersections = vec![Intersection::default()];

    while !intersections.is_empty() {
        for int in intersections.iter() {
            rooms[int.id].apply_dir(int.dir, EXPLODE_SPEED);
        }

        intersections.clear();

        for rm_a in 0..rm_len {
            for rm_b in 0..rm_len {
                if rm_a == rm_b { break }

                let room_a = &rooms[rm_a];
                let room_b = &rooms[rm_b];

                let aabb_a = Aabb {
                    min: room_a.aabb.min + room_a.position,
                    max: room_a.aabb.max + room_a.position,
                };
                let aabb_b = Aabb { 
                    min: room_b.aabb.min + room_b.position,
                    max: room_b.aabb.max + room_b.position,
                };

                if aabb_coll(aabb_a, aabb_b) {
                    let dir = (room_b.position.cast_float() - room_a.position.cast_float()).normalized();
                    intersections.push(Intersection { id: rm_b, dir });
                }
            }
        }
    }
}