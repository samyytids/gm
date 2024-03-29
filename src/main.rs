use bevy::prelude::*;

mod setup;
mod animate;
mod movement;
mod resources;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(resources::TimeTaken::default()) // prevents blurry sprites
        .add_systems(Startup, setup::setup)
        .add_systems(Update, (
                animate::animate_sprite, 
                movement::set_player_direction,
                animate::update_animation.before(animate::animate_sprite),
                movement::character_movement.after(movement::set_player_direction),
            ), 
        )
        .run();
}
