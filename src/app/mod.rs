//! Shows how to render simple primitive shapes with a single color.

mod sys;

use bevy::prelude::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, sys::setup)
        .add_systems(Update, sys::update)
        .run();
}
