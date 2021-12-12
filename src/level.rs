use crate::{
    camera::CameraTarget,
    loading_state::TextureAssets,
    player::{GridPosition, MoveDirection, Player, Speed, PLAYER_SPEED},
    GameState,
};
use bevy::prelude::*;

pub const CELL_WIDTH: f32 = 32.0;
pub const CELL_HEIGHT: f32 = 32.0;

pub struct Wall;

pub struct Level {
    width: usize,
    height: usize,
    walls: Vec<bool>,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Level {
        Self {
            width,
            height,
            walls: vec![false; width * height],
        }
    }

    pub fn set_wall(&mut self, x: usize, y: usize) {
        let index = self.xy_to_index(x, y);
        self.walls[index] = true;
    }

    #[allow(dead_code)]
    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        let index = self.xy_to_index(x, y);
        self.walls[index]
    }

    #[inline]
    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width);
        assert!(y < self.height);
        y * self.width + x
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Gameplay).with_system(spawn_level_system.system()),
        );
    }
}

// TODO: proper level format (json maybe?)
// TODO: load level as an asset
fn spawn_level_system(
    mut cmd: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let path = "assets/levels/level1.txt";
    let data = std::fs::read_to_string(path).unwrap(); // TODO: Handle errors

    let wall_material = materials.add(textures.wall.clone().into());
    let player_material = materials.add(textures.player.clone().into());

    let mut lines_iter = data.lines();
    let mut level = {
        // TODO: Handle errors
        let line = lines_iter.next().unwrap();
        let mut v = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let width = v.next().unwrap();
        let height = v.next().unwrap();
        Level::new(width, height)
    };

    for (j, line) in lines_iter.enumerate() {
        for (i, c) in line.chars().enumerate() {
            let x = i as f32 * CELL_WIDTH;
            let y = -(j as f32 * CELL_HEIGHT);
            match c {
                '#' => {
                    level.set_wall(i, j);
                    spawn_wall(&mut cmd, wall_material.clone(), x, y);
                }
                'P' => spawn_player(&mut cmd, player_material.clone(), x, y),
                _ => {}
            }
        }
    }
}

fn spawn_wall(cmd: &mut Commands, material: Handle<ColorMaterial>, x: f32, y: f32) {
    cmd.spawn_bundle(SpriteBundle {
        material,
        transform: Transform::from_xyz(x, y, 0.0),
        ..Default::default()
    })
    .insert(Wall);
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
    .insert(CameraTarget::new(-50.0, 50.0, -50.0, 50.0, 0.2))
    .insert(MoveDirection::Right);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_index() {
        let level = Level::new(100, 20);
        let index = level.xy_to_index(0, 2);
        assert_eq!(index, 200);

        let index = level.xy_to_index(2, 3);
        assert_eq!(index, 302);

        let index = level.xy_to_index(99, 0);
        assert_eq!(index, 99);

        let index = level.xy_to_index(0, 1);
        assert_eq!(index, 100);
    }

    #[test]
    fn level_walls() {
        let mut level = Level::new(100, 20);
        let x = 0;
        let y = 0;
        level.set_wall(x, y);
        assert!(level.is_wall(x, y));

        let x = 90;
        let y = 11;
        level.set_wall(x, y);
        assert!(level.is_wall(x, y));
    }
}
