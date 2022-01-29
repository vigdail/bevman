use crate::{
    level::{CELL_HEIGHT, CELL_WIDTH},
    player::{GridPosition, Player},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct DebugCell;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_debug_cell_system)
            .add_system(debug_cell_position_system);
    }
}

fn create_debug_cell_system(mut cmd: Commands) {
    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(CELL_WIDTH, CELL_HEIGHT)),
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
