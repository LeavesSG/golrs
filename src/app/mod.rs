//! Shows how to render simple primitive shapes with a single color.

mod sys;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, sys::setup)
        .run();
}
