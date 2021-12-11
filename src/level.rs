use crate::{
    loading_state::TextureAssets,
    player::{GridPosition, MoveDirection, Player, Speed, CELL_HEIGHT, CELL_WIDTH, PLAYER_SPEED},
    GameState,
};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Gameplay).with_system(spawn_level_system.system()),
        );
    }
}

fn spawn_level_system(
    mut cmd: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let path = "assets/levels/level1.txt";
    let data = std::fs::read_to_string(path).unwrap(); // TODO: Handle errors

    let wall_material = materials.add(ColorMaterial::color(Color::BLUE));
    let player_material = materials.add(textures.player.clone().into());

    for (j, line) in data.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            let x = i as f32 * CELL_WIDTH;
            let y = -(j as f32 * CELL_HEIGHT);
            match c {
                '#' => spawn_wall(&mut cmd, wall_material.clone(), x, y),
                'P' => spawn_player(&mut cmd, player_material.clone(), x, y),
                _ => {}
            }
        }
    }
}

fn spawn_wall(cmd: &mut Commands, material: Handle<ColorMaterial>, x: f32, y: f32) {
    cmd.spawn_bundle(SpriteBundle {
        material,
        sprite: Sprite {
            size: Vec2::new(CELL_WIDTH, CELL_HEIGHT),
            ..Default::default()
        },
        transform: Transform::from_xyz(x, y, 0.0),
        ..Default::default()
    });
}

fn spawn_player(cmd: &mut Commands, material: Handle<ColorMaterial>, x: f32, y: f32) {
    cmd.spawn_bundle(SpriteBundle {
        material,
        transform: Transform::from_xyz(x, y, 0.0),
        ..Default::default()
    })
    .insert(GridPosition::default())
    .insert(Player)
    .insert(Speed(PLAYER_SPEED))
    .insert(MoveDirection::Right);
}
