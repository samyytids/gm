use bevy::prelude::*;

mod setup;
mod animate;
mod movement;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup::setup)
        .add_systems(Update, (
                animate::animate_sprite, 
                movement::set_player_direction,
                animate::update_animation.before(animate::animate_sprite),
                movement::character_movement.after(movement::set_player_direction),
                movement::check_something,
            ), 
        )
        .run();
}
