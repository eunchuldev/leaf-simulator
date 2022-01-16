mod shape;
mod mesh;
mod bevy_components;
mod lsystem;
mod error;
mod pot;

//use mesh::render_meshes;
//use bevy::render::mesh::Mesh;
use bevy::prelude::*;
//pub use component::{Component, render_components};
use bevy_components::{PlantComponentDatas, render_component_datas}; 
use bevy_rapier3d::prelude::*;
pub use shape::PlantShape;
pub use lsystem::{ILSystem, LElement, LRules};
pub use error::PlantError;
pub use pot::Pot;

pub struct Plant {
    lsystem: ILSystem,
    shape: PlantShape,
    pot_entities: Vec<Entity>,
    body_entities: Vec<Entity>,
    joint_entities: Vec<Entity>,
    pot: Pot,
}

impl Plant {
    pub fn new(axiom: String, rules: LRules, shape: PlantShape, pot: Pot) -> Self {
        let lsystem = ILSystem::new(axiom, rules);
        //let component_datas = render_component_datas(lsystem.production(), &shape);
        Self {
            lsystem,
            shape,
            pot,
            pot_entities: Vec::new(),
            body_entities: Vec::new(),
            joint_entities: Vec::new(),
        }
    }
    pub fn iterate(&mut self, mut commands: &mut Commands) {
        self.lsystem.next();
        //self.component_datas = render_component_datas(self.lsystem.next(), &self.shape);
    }
    pub fn spawn(
        &mut self, 
        mut commands: &mut Commands,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Result<(), PlantError> {
        let component_datas = render_component_datas(self.lsystem.production(), &self.shape);
        if self.body_entities.len() > 0 {
            Err(PlantError::EntityAlreadyExists)
        } else {
            let pot_entity = self.pot.spawn(commands, meshes, materials);
            self.pot_entities.push(pot_entity);
            //let mut e = commands.spawn();
            for data in component_datas.bodies.into_iter() {
                let e = commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(data.mesh.clone()),
                    material: materials.add(data.material.clone()),
                    transform: data.transform,
                    ..Default::default()
                })
                .insert_bundle(data.rigid_body)
                .insert_bundle(data.collider)
                .insert(ColliderPositionSync::Discrete)
                .id();
                self.body_entities.push(e);
            }
            for data in component_datas.joints.into_iter() {
                let e = commands.spawn().insert(JointBuilderComponent::new(data.joint, self.body_entities[data.parent_index], self.body_entities[data.child_index])).id();
                self.joint_entities.push(e);
            }
            //self.entity = Some(e.id());
            Ok(())
        }
    }
}
