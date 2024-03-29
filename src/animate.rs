use bevy::prelude::*;

use std::time::Duration;
use crate::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub indices: Vec<usize>,
    pub tracker: usize,
}

impl AnimationIndices {
    pub fn new(
        indices: Vec<usize>
    ) -> Self {
        AnimationIndices{
            indices,
            tracker: 0
        }
    }

    pub fn _update_indices(
        &mut self,
        indices: Vec<usize>,
    ) { 
        self.indices = indices;
    }
}

#[derive(Component)]
pub struct WalkingAnimations {
    pub up1: AnimationIndices,
    pub up2: AnimationIndices,
    pub left: AnimationIndices,
    pub down1: AnimationIndices,
    pub down2: AnimationIndices,
    pub right: AnimationIndices,
    pub active_animation: movement::Direction,
    pub step: u8,
}

impl WalkingAnimations {
    pub fn new(
        up1: AnimationIndices,
        up2: AnimationIndices,
        left: AnimationIndices,
        down1: AnimationIndices,
        down2: AnimationIndices,
        right: AnimationIndices,
    ) -> Self {
        WalkingAnimations {
            up1,
            up2,
            left,
            down1,
            down2,
            right,
            active_animation: movement::Direction::DOWN,
            step: 0,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

fn advance_animation(
    atlas: &mut TextureAtlas,
    indices: &mut AnimationIndices,
) {
    atlas.index = if indices.tracker == indices.indices.len() - 1 {
        indices.tracker = 1;
        indices.indices[indices.tracker]
        
    } else {
        indices.tracker =  indices.tracker + 1;
        indices.indices[indices.tracker]
    };
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
            &mut WalkingAnimations, 
            &mut AnimationTimer, 
            &mut TextureAtlas
        ),
        With<movement::Moving>,
    >,
) {
    for (
        mut indices, 
        mut timer, 
        mut atlas
    ) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            match indices.active_animation {
                movement::Direction::UP => {
                    if indices.step == 0 {
                        advance_animation(&mut atlas, &mut indices.up1)
                    } else {
                        advance_animation(&mut atlas, &mut indices.up2)
                    }
                    
                },
                movement::Direction::LEFT => {
                    advance_animation(&mut atlas, &mut indices.left)
                },
                movement::Direction::DOWN => {
                    if indices.step == 0 {
                        advance_animation(&mut atlas, &mut indices.down1)
                    } else {
                        advance_animation(&mut atlas, &mut indices.down2)
                    }
                },
                movement::Direction::RIGHT => {
                    advance_animation(&mut atlas, &mut indices.right)
                },
            }
        }
    }
}

fn set_animation(
    indices: &mut WalkingAnimations,
    atlas: &mut TextureAtlas,
    direction: movement::Direction,
    timer: &mut AnimationTimer,
) {
    let animation_length = (16.0*6.0)/150.0;
    match direction {
        movement::Direction::UP => {
            if indices.step == 0 {
                indices.up1.tracker = 0;
                atlas.index = indices.up1.indices[indices.up1.tracker];
                timer.set_duration(Duration::from_secs_f32(animation_length/indices.up1.indices.len() as f32));
                indices.step = 1;
            } else {
                indices.up2.tracker = 0;
                atlas.index = indices.up2.indices[indices.up2.tracker];
                timer.set_duration(Duration::from_secs_f32(animation_length/indices.up2.indices.len() as f32));
                indices.step = 0;
            }
        },
        movement::Direction::LEFT => {
            indices.left.tracker = 0;
            atlas.index = indices.left.indices[indices.left.tracker];
            timer.set_duration(Duration::from_secs_f32(animation_length/indices.left.indices.len() as f32));
            indices.step = 0;
        },
        movement::Direction::DOWN => {
            if indices.step == 0 {
                indices.down1.tracker = 0;
                atlas.index = indices.down1.indices[indices.down1.tracker];
                timer.set_duration(Duration::from_secs_f32(animation_length/indices.down1.indices.len() as f32));
                indices.step = 1;
            } else {
                indices.down2.tracker = 0;
                atlas.index = indices.down2.indices[indices.down2.tracker];
                timer.set_duration(Duration::from_secs_f32(animation_length/indices.down2.indices.len() as f32));
                indices.step = 0;
            }
        },
        movement::Direction::RIGHT => {
            indices.right.tracker = 0;
            atlas.index = indices.right.indices[indices.right.tracker];
            timer.set_duration(Duration::from_secs_f32(animation_length/indices.right.indices.len() as f32));
            indices.step = 0;
        },

    }
    indices.active_animation = direction;
    
}

pub fn update_animation(
    mut query: Query<(
            &mut WalkingAnimations,
            &mut TextureAtlas, 
            &movement::Direction,
            &mut AnimationTimer,
        ),
        Added<movement::Moving>,
    >,
) {
    for (
        mut indices,
        mut atlas, 
        &direction,
        mut timer,
    ) in &mut query {
        match direction {
            movement::Direction::UP => {
                set_animation(&mut indices, &mut atlas, direction, &mut timer);
            },
            movement::Direction::LEFT => {
                set_animation(&mut indices, &mut atlas, direction, &mut timer);
            },
            movement::Direction::DOWN => {
                set_animation(&mut indices, &mut atlas, direction, &mut timer);
            },
            movement::Direction::RIGHT => {
                set_animation(&mut indices, &mut atlas, direction, &mut timer);
            },
        }
    }
}