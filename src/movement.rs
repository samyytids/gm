use core::fmt;

use bevy::prelude::*;
use bevy::input::*;

use crate::*;

#[derive(Component, Clone, Copy)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Component, Copy, Clone)]
pub struct Moving{
    pub distance: f32,
}

impl Moving {
    pub fn default() -> Self {
        Moving {
            distance: 0.0,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::UP => write!(f, "UP!"),
            Direction::DOWN => write!(f, "DOWN!"),
            Direction::LEFT => write!(f, "LEFT!"),
            Direction::RIGHT => write!(f, "RIGHT!"),
        }
    }
}

pub fn set_player_direction(
    mut keys: EventReader<keyboard::KeyboardInput>,
    mut commands: Commands,
    player: Query<Entity, (With<setup::Player>, Without<Moving>)>,
) {
    for entity in player.iter() {
        let mut direction: Option<Direction> = None;
        for key_press in keys.read() {
            direction = match (key_press.state, key_press.key_code) {
                (ButtonState::Pressed, KeyCode::KeyW) => Some(Direction::UP),
                (ButtonState::Pressed, KeyCode::KeyA) => Some(Direction::LEFT),
                (ButtonState::Pressed, KeyCode::KeyS) => Some(Direction::DOWN),
                (ButtonState::Pressed, KeyCode::KeyD) => Some(Direction::RIGHT),
                _ => None,
            };
            if direction.is_some() {
                break;
            }
        }
        if direction.is_some() {
            commands.entity(entity)
                    .insert(direction.unwrap())
                    .insert(Moving::default());
        }
    }
}   

pub fn character_movement(
    mut player: Query<(
            &mut Transform,
            &mut Moving,
            &Direction,
            Entity,
        ),
        With<setup::Player>,
    >,
    mut camera: Query<
        &mut Transform,
        (With<Camera>, Without<setup::Player>),
    >,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (
        mut transform,
        mut moving,
        direction,
        entity,
    ) in player.iter_mut() {  

        let mut movement_amount = 150.0*time.delta_seconds();

        if moving.distance == 16.0 {
            commands.entity(entity).remove::<Moving>();
        }

        moving.distance += movement_amount;

        if moving.distance > 16.0*6.0 {
            movement_amount = movement_amount - moving.distance % 16.0;
            moving.distance = 16.0;
        }
        let mut camera_transform = camera.single_mut();

        match direction {
            Direction::UP => {
                transform.translation.y += movement_amount;
                camera_transform.translation.y += movement_amount;
            },
            Direction::LEFT => {
                transform.translation.x -= movement_amount;
                camera_transform.translation.x -= movement_amount;
            },
            Direction::DOWN => {
                transform.translation.y -= movement_amount;
                camera_transform.translation.y -= movement_amount;
            },
            Direction::RIGHT => {
                transform.translation.x += movement_amount;
                camera_transform.translation.x += movement_amount;
            },
        }
    }
}