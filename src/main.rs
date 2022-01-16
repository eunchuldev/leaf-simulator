mod plant;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use plant::{Plant, PlantShape, Pot};

/// This example shows how to configure Physically Based Rendering (PBR) parameters.
fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
/*        .insert_resource(RapierConfiguration {
            gravity: -Vector::y(),
            ..Default::default()
        })*/
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    //mut materials: ResMut<Assets<ColorMaterial>>,
) {
    rapier_config.gravity = -bevy_rapier3d::na::Vector3::y()*100.0;
    let mut plant = Plant::new("B".to_string(), [
        ('B', "BBB[+B][|+B]B".chars().collect())
    ].into_iter().collect(), PlantShape {
        unit_length: 10.0,
        unit_width: 2.0,
        unit_angle: 0.25,
        width_growth_factor: 0.2,
        branch_color: [1.0, 0.8, 0.7, 1.0],
    }, Pot::PETBottle(0));
    plant.iterate(&mut commands);
    plant.iterate(&mut commands);
    //plant.iterate(&mut commands);
    plant.spawn(&mut commands, &mut meshes, &mut materials).unwrap();
    // add entities to the world
    /*for y in -2..=2 {
        for x in -5..=5 {
            let x01 = (x + 5) as f32 / 10.0;
            let y01 = (y + 2) as f32 / 4.0;
            // sphere
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.45,
                    subdivisions: 32,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::hex("ffd891").unwrap(),
                    // vary key PBR parameters on a grid of spheres to show the effect
                    metallic: y01,
                    perceptual_roughness: x01,
                    ..Default::default()
                }),
                transform: Transform::from_xyz(x as f32, y as f32 + 0.5, 0.0),
                ..Default::default()
            });
        }
    }*/
    // light
    /*commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(50.0, 50.0, 50.0)),
        point_light: PointLight {
            intensity: 600000.,
            range: 100.,
            ..Default::default()
        },
        ..Default::default()
    });*/
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(10.0, 2.0, 10.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0))
            .looking_at(Vec3::default(), Vec3::Y),
        orthographic_projection: OrthographicProjection {
            scale: 0.5,
            ..Default::default()
        },
        ..OrthographicCameraBundle::new_3d()
    });
    //commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle::default())
        .insert_bundle(ColliderBundle {
            position: [50.0 / 2.0, 50.0 / 2.0, 0.0].into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
}
/*
use bevy::prelude::*;
use bevy::prelude::shape;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::na::Vector3;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Player Movement Example".to_string(),
            width: 1000.0,
            height: 1000.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player.system())
        .add_system(player_movement.system())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}

// The float value is the player movement speed in 'pixels/second'.
#[derive(Component)]
struct Player(f32);

fn spawn_player(
    mut commands: Commands, 
    mut rapier_config: ResMut<RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
    // Set gravity to 0.0 and spawn camera.
    rapier_config.gravity = -Vector3::y()*100.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(10.0, 2.0, 10.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 8.0))
            .looking_at(Vec3::default(), Vec3::Y),
        orthographic_projection: OrthographicProjection {
            scale: 0.01,
            ..Default::default()
        },
        ..OrthographicCameraBundle::new_3d()
    });

/*    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());*/

    let sprite_size_x = 40.0;
    let sprite_size_y = 40.0;

    // While we want our sprite to look ~40 px square, we want to keep the physics units smaller
    // to prevent float rounding problems. To do this, we set the scale factor in RapierConfiguration
    // and divide our sprite_size by the scale.
    rapier_config.scale = 20.0;
    let collider_size_x = sprite_size_x / rapier_config.scale;
    let collider_size_y = sprite_size_y / rapier_config.scale;

    // Spawn entity with `Player` struct as a component for access in movement query.
    commands
        .spawn()
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: sprite_size_x })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            //transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle::default())
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(30.0, 30.0, 30.0).into(),
            position: [collider_size_x / 2.0, collider_size_y / 2.0, 0.0].into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        //.insert(ColliderDebugRender::with_id(0))
        .insert(Player(300.0));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    mut player_info: Query<(&Player, &mut RigidBodyVelocityComponent)>,
) {
    for (player, mut rb_vels) in player_info.iter_mut() {
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vector3::new(x_axis as f32, y_axis as f32, 0.0);
        if move_delta != Vector3::zeros() {
            // Note that the RapierConfiguration::Scale factor is also used here to transform
            // the move_delta from: 'pixels/second' to 'physics_units/second'
            move_delta /= move_delta.magnitude() * rapier_parameters.scale;
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        if left || down || left || right {
            rb_vels.linvel = move_delta * player.0;
        }
    }
}
*/
