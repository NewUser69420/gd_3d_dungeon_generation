use godot::prelude::*;

mod dungeon;
mod voxel;
mod helper;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
