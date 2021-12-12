use crate::{
    level::{CELL_HEIGHT, CELL_WIDTH},
    GameState,
};
use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};

pub const PLAYER_SPEED: f32 = 50.0;

pub struct Player;

pub struct Speed(pub f32);

#[derive(Copy, Clone, Debug)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

impl MoveDirection {
    pub fn to_vec3(self) -> Vec3 {
        match self {
            MoveDirection::Up => Vec3::Y,
            MoveDirection::Down => -Vec3::Y,
            MoveDirection::Left => -Vec3::X,
            MoveDirection::Right => Vec3::X,
        }
    }

    pub fn is_opposite(&self, other: &MoveDirection) -> bool {
        matches!(
            (self, other),
            (MoveDirection::Up, MoveDirection::Down)
                | (MoveDirection::Down, MoveDirection::Up)
                | (MoveDirection::Left, MoveDirection::Right)
                | (MoveDirection::Right, MoveDirection::Left),
        )
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct NextDirection(Option<MoveDirection>);

#[derive(Default)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(NextDirection::default())
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(world_to_grid_system.system())
                    .with_system(input_system.system())
                    .with_system(movement_system.system())
                    .with_system(change_direction_system.system()),
            );
    }
}

fn world_to_grid_system(mut query: Query<(&Transform, &mut GridPosition)>) {
    for (transform, mut position) in query.iter_mut() {
        let x = (transform.translation.x / CELL_WIDTH).round() as i32;
        let y = (transform.translation.y / CELL_HEIGHT).round() as i32;
        *position = GridPosition { x, y };
    }
}

fn movement_system(time: Res<Time>, mut query: Query<(&mut Transform, &MoveDirection, &Speed)>) {
    for (mut transform, direction, speed) in query.iter_mut() {
        transform.translation += direction.to_vec3() * speed.0 * time.delta_seconds();
    }
}

fn change_direction_system(
    mut query: Query<(&mut MoveDirection, &Transform, &MoveDirection), With<Player>>,
    mut next_direction: ResMut<NextDirection>,
) {
    if let Some(dir) = next_direction.0 {
        for (mut direction, transform, current_direction) in query.iter_mut() {
            let is_x_aligned = (transform.translation.x as i32 % CELL_WIDTH as i32) == 0;
            let is_y_aligned = (transform.translation.y as i32 % CELL_HEIGHT as i32) == 0;
            let is_opposite = current_direction.is_opposite(&dir);
            if is_opposite || (is_x_aligned && is_y_aligned) {
                *direction = dir;
                next_direction.0 = None;
            }
        }
    }
}

fn input_system(mut input: EventReader<KeyboardInput>, mut next_direction: ResMut<NextDirection>) {
    for event in input.iter() {
        if event.state != ElementState::Pressed {
            continue;
        }

        let dir = if let Some(key_code) = event.key_code {
            match key_code {
                KeyCode::W => Some(MoveDirection::Up),
                KeyCode::S => Some(MoveDirection::Down),
                KeyCode::A => Some(MoveDirection::Left),
                KeyCode::D => Some(MoveDirection::Right),
                _ => None,
            }
        } else {
            None
        };

        next_direction.0 = dir;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_to_grid() {
        let mut world = World::default();
        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(world_to_grid_system.system());

        let player = world
            .spawn()
            .insert(Transform::from_xyz(10.0, 64.0, 0.0))
            .insert(GridPosition::default())
            .id();

        let enemy = world
            .spawn()
            .insert(Transform::from_xyz(-32.0, 640.0, 0.0))
            .insert(GridPosition::default())
            .id();

        update_stage.run(&mut world);

        let player_pos = world.get::<GridPosition>(player);
        assert!(player_pos.is_some());
        let player_pos = player_pos.unwrap();
        assert_eq!(player_pos.x, 0);
        assert_eq!(player_pos.y, 2);

        let enemy_pos = world.get::<GridPosition>(enemy);
        assert!(enemy_pos.is_some());
        let enemy_pos = enemy_pos.unwrap();
        assert_eq!(enemy_pos.x, -1);
        assert_eq!(enemy_pos.y, 20);
    }

    #[test]
    fn direction_opposite() {
        let dir_left = MoveDirection::Left;
        let dir_right = MoveDirection::Right;
        let dir_up = MoveDirection::Up;
        let dir_down = MoveDirection::Down;

        assert!(dir_left.is_opposite(&dir_right));
        assert!(!dir_left.is_opposite(&dir_left));
        assert!(!dir_left.is_opposite(&dir_up));
        assert!(!dir_left.is_opposite(&dir_down));

        assert!(dir_down.is_opposite(&dir_up));
        assert!(!dir_down.is_opposite(&dir_down));
    }
}
