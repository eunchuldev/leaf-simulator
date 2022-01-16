use bevy::prelude::*;
use bevy::prelude::shape;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::na::Point3;

pub enum Pot {
    PETBottle(usize),
}

impl Pot {
    pub fn spawn(
        &mut self, 
        mut commands: &mut Commands,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Entity {
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: 30.0,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("557799").unwrap(),
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, -15.0, 0.0),
            ..Default::default()
        }).insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(30.0, 30.0, 30.0).into(),
            position: Point3::new(0.0, -30.0, 0.0).into(),
            /*position: ColliderPositionComponent(ColliderPosition(
                    Isometry {
                        rotation: Rotation3::from_matrix_unchecked(rotation_matrix.into()).into(),
                        translation: Translation3::new(u.x*0.5, u.y*0.5, u.z*0.5),
                        ..Default::default()
                    }
            )),*/
            ..Default::default()
        }).id()
        /*.insert_bundle(data.collider)
          .insert_bundle(data.rigid_body);*/
    }

}
