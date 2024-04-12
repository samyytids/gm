use bevy::prelude::*;
use crate::*;

use self::animate::{AnimationIndices, WalkingAnimations};

#[derive(Component)]
pub struct Player;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut text_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> = asset_server.load("./characters.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(16.0), 10, 75, None, None);
    let texture_atlas_layout = text_atlas_layouts.add(layout);
    let female: bool = true;
    
    let mut up1 = AnimationIndices::new(vec![3, 4]);
    let mut up2 = AnimationIndices::new(vec![5, 4]);
    let mut left = AnimationIndices::new(vec![7, 6]);
    let mut down1 = AnimationIndices::new(vec![0, 1]);
    let mut down2 = AnimationIndices::new(vec![2, 1]);
    let mut right = AnimationIndices::new(vec![9, 8]);

    if female {
        up1 = AnimationIndices::new(vec![33, 34]);
        up2 = AnimationIndices::new(vec![35, 34]);
        left = AnimationIndices::new(vec![37, 36]);
        down1 = AnimationIndices::new(vec![30, 31]);
        down2 = AnimationIndices::new(vec![32, 31]);
        right = AnimationIndices::new(vec![39, 38]);
    }
    

    let walking_animations = WalkingAnimations::new(
        up1, up2, left, down1, down2, right
    );

    commands.spawn(Camera2dBundle::default());

    let bg: Handle<Image> = asset_server.load("tiledbg.png");
    commands.spawn((SpriteBundle {
        texture:  bg,
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    },));

    commands.spawn((
        SpriteSheetBundle {
            texture, 
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: walking_animations.down1.indices[1],
            },
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        walking_animations,
        animate::AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        Player,
    ));

    let texture: Handle<Image> = asset_server.load("./pokemon.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(16.0), 10, 13, None, None);
    let texture_atlas_layout = text_atlas_layouts.add(layout);
    let up1 = AnimationIndices::new(vec![3, 4]);
    let up2 = AnimationIndices::new(vec![5, 4]);
    let left = AnimationIndices::new(vec![7, 6]);
    let down1 = AnimationIndices::new(vec![0, 1]);
    let down2 = AnimationIndices::new(vec![2, 1]);
    let right = AnimationIndices::new(vec![9, 8]);    

    let walking_animations = WalkingAnimations::new(
        up1, up2, left, down1, down2, right
    );
    commands.spawn((
        SpriteSheetBundle {
            texture, 
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: walking_animations.down1.indices[1],
            },
            transform: Transform {
                translation: Vec3::new(32.0*6.0, 32.0*6.0, 0.1),
                scale: Vec3::splat(6.0),
                rotation: Quat::from_vec4(Vec4::splat(0.0)),
            },
            ..default()
        },
        walking_animations,
        animate::AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));
}