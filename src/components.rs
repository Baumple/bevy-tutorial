use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BulletLifeTime {
    pub timer: Timer,
}