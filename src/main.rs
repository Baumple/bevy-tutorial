mod assets;
mod components;

use std::f32::consts::PI;

use assets::GameAssets;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use components::{BulletLifeTime, Tower};

const HEIGHT: f32 = 720.0;
const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            height: HEIGHT,
            width: WIDTH,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .register_type::<Tower>()
        .add_startup_system(load_assets)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_system(tower_shooting)
        .add_system(bullet_despawn)
        .run();
}
fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<&mut Tower>,
    bullet_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform =
                Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));
            commands
                .spawn_bundle(SceneBundle {
                    scene: bullet_assets.bullet_scene.clone(),
                    transform: spawn_transform,
                    ..Default::default()
                })
                .insert(BulletLifeTime {
                    timer: Timer::from_seconds(0.5, false),
                })
                .insert(Name::new("Bullet"));
        }
    }
}

fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets
            .load("D:\\Programming\\Rust\\bevy_tutorial\\assets\\Donuts.glb#Scene0"),
    });
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut BulletLifeTime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.2, 0.7, 0.2).into()),
            ..Default::default()
        })
        .insert(Name::new("Ground"));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.6, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, true),
        })
        .insert(Name::new("Tower"));

    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 2000.0,
                shadows_enabled: true,
                radius: 0.5,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 10.0, 6.0),
            ..Default::default()
        })
        .insert(Name::new("Light"));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
