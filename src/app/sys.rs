use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::gol::{game::GameOfLife, mech::snapshot::Snapshot};

const ALIVE_COLOR: [f32; 3] = [0., 0., 0.];
const DEAD_COLOR: [f32; 3] = [1., 1., 1.];
const CELL_WIDTH: f32 = 10.;
const CELL_HEIGHT: f32 = 10.;

unsafe impl Sync for GameOfLife {}
unsafe impl Send for GameOfLife {}
impl Resource for GameOfLife {}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut game = GameOfLife::default();
    let width = game.cfg.width as f32;
    let height = game.cfg.height as f32;
    let xt = width * CELL_WIDTH;
    let yt = height * CELL_HEIGHT;

    commands.spawn(Camera2dBundle::default());

    let size = game.board.buf_size();
    let snapshot = Snapshot::random(size);
    for (index, item) in snapshot.buf.iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::rgb_from_array(if *item { ALIVE_COLOR } else { DEAD_COLOR });
        let reflection = game.board.decount_vec(index);
        let i = reflection[0];
        let j = reflection[1];
        let bundle = MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(CELL_WIDTH, CELL_HEIGHT))),
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT to +X_EXTENT.
                -xt / 2. + i as f32 / (width - 1.) * xt,
                -yt / 2. + j as f32 / (height - 1.) * yt,
                0.0,
            ),
            ..default()
        };
        commands.spawn(bundle);
    }
    game.board.consume_snapshot(snapshot);

    commands.insert_resource(game);
}

pub fn update(
    mat_handles: Query<&Handle<ColorMaterial>>,
    mut game: ResMut<GameOfLife>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let snapshot = game.board.next_snapshot(&game.rule);
    for (i, handle) in mat_handles.iter().enumerate() {
        if let Some(material) = materials.get_mut(handle) {
            let alive = snapshot.buf[i];
            let color = Color::rgb_from_array(if alive { ALIVE_COLOR } else { DEAD_COLOR });
            material.color = color;
        }
    }
    game.board.consume_snapshot(snapshot);
}
