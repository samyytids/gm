use core::fmt;

use bevy::prelude::*;
use crate::*;

use self::animate::{AnimationTimer, WalkingAnimations};

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
    pub finished: bool,
}

impl Moving {
    pub fn default() -> Self {
        Moving {
            distance: 0.0,
            finished: false,
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

pub fn check_something(
    query: Query<(&Visibility, &WalkingAnimations, &Transform)>
) {
    
    for (_, _, transform) in query.iter() {
        println!("x: {}, y: {}, z: {}", transform.translation.x, transform.translation.y, transform.translation.z);
    }
}

pub fn set_player_direction(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    player: Query<Entity, (With<setup::Player>, Without<Moving>)>,
) {
    for entity in player.iter() {
        let mut direction: Option<Direction> = None;
        if keys.just_pressed(KeyCode::KeyW) 
            || keys.pressed(KeyCode::KeyW) {

            direction = Some(Direction::UP);

        } else if keys.just_pressed(KeyCode::KeyA) 
            || keys.pressed(KeyCode::KeyA) {

            direction = Some(Direction::LEFT);

        } else if keys.just_pressed(KeyCode::KeyS) 
            || keys.pressed(KeyCode::KeyS) {

            direction = Some(Direction::DOWN);

        } else if keys.just_pressed(KeyCode::KeyD) 
            || keys.pressed(KeyCode::KeyD) {
            direction = Some(Direction::RIGHT);

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
            &mut AnimationTimer,
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
        mut timer,
        entity,
    ) in player.iter_mut() {  
        let mut movement_amount = 150.0*time.delta_seconds();

        if moving.finished {
            commands.entity(entity).remove::<Moving>();
            timer.reset();
        }

        moving.distance += movement_amount;

        if moving.distance > 16.0*6.0 {
            movement_amount = movement_amount - moving.distance % 16.0;
            moving.finished = true;
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