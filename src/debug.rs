use crate::{
    level::{CELL_HEIGHT, CELL_WIDTH},
    player::{GridPosition, Player},
};
use bevy::prelude::*;

pub struct DebugCell;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_debug_cell_system.system())
            .add_system(debug_cell_position_system.system());
    }
}

fn create_debug_cell_system(mut cmd: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    cmd.spawn_bundle(SpriteBundle {
        material: materials.add(ColorMaterial::color(Color::GREEN)),
        sprite: Sprite {
            size: Vec2::new(CELL_WIDTH, CELL_HEIGHT),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(DebugCell);
}

fn debug_cell_position_system(
    mut cell_query: Query<&mut Transform, With<DebugCell>>,
    player_query: Query<&GridPosition, With<Player>>,
) {
    for (mut cell_transform, player_position) in cell_query.iter_mut().zip(player_query.iter()) {
        let x = player_position.x as f32 * CELL_WIDTH;
        let y = player_position.y as f32 * CELL_HEIGHT;
        cell_transform.translation = Vec3::new(x, y, 0.0);
    }
}
