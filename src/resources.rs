use bevy::prelude::*;


#[derive(Resource, Default)]
pub struct TimeTaken {
    pub start: f32,
}

impl TimeTaken {
    pub fn _time_take(&self, time: f32) {
        println!("{}", time-self.start);
    }
}