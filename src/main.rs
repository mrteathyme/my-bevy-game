use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::f32::consts::PI;

pub const HEIGHT: f32 = 1080.0;
pub const WIDTH: f32 = 2560.0;

#[derive(Component,Reflect)]
pub struct Tower {
    shooting_timer: Timer,
}

#[derive(Component,Reflect)]
pub struct Lifetime {
    timer: Timer,
}


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.3,0.1,0.2)))

    .add_startup_system(spawn_basic_scene)
    .add_startup_system(spawn_camera)
    .add_system(tower_shooting)
    .add_system(bullet_despawn)
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "My Bevy Game".to_string(),
            //resizable: false,
            ..default()
        },
        ..default()
    }))
    .add_plugin(WorldInspectorPlugin)
    .register_type::<Tower>()
    .register_type::<Lifetime>()
    .run();
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,


) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {size: 5.0})),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(Name::new("Ground"));
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {size: 1.0})),
        material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
        transform: Transform::from_xyz(0.0,0.5,0.0),
        ..default()
    })
    .insert(Tower {
        shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating)
    })
    .insert(Name::new("Tower"));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    })
    .insert(Name::new("Tower"));
}

fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<&mut Tower>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform = 
                Transform::from_xyz(0.0,0.7,0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));
        
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {size: 0.1})),
            material: materials.add(Color::rgb(0.87, 0.44, 0.42).into()),
            transform: spawn_transform,
            ..default()
        })
        .insert(Lifetime {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating)
        })
        .insert(Name::new("Bullet"));
    }
    }
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime )in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}