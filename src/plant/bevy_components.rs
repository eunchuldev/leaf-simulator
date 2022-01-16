use super::LElement;
use super::PlantShape;
//use bevy_prototype_lyon::geometry::Geometry;
use bevy::{
    math::{Vec3, Vec2, Mat3, Vec3Swizzles},
    render::{
        mesh::{Indices, Mesh},
        render_resource::PrimitiveTopology,
    },
};
use bevy::prelude::shape;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::na::{Rotation3, Translation3};

fn normal(p1: Vec3, p2: Vec3, p3: Vec3) -> Vec3 {
    let dir = (p2 - p1).cross(p3 - p1);
    dir.normalize()
}

pub struct PlantBodyData {
    pub mesh: Mesh,
    pub transform: Transform,
    pub material: StandardMaterial,
    pub collider: ColliderBundle, 
    pub rigid_body: RigidBodyBundle,
}

pub struct PlantJointData {
    pub joint: FixedJoint,
    pub parent_index: usize,
    pub child_index: usize,
}

pub struct PlantComponentDatas {
    pub bodies: Vec<PlantBodyData>,
    pub joints: Vec<PlantJointData>,
}

fn _render_component_datas(
    components: &mut PlantComponentDatas,
    elements: &[LElement], 
    mut index: usize, 
    mut pos: Vec3, 
    mut rotation_matrix: Mat3, 
    render_option: &PlantShape,
) -> Option<usize> {
    let mut returning_index = None; 
    let mut last_body_index = 0;
    //let mut last_body_width = 0.0;
    while index < elements.len() {
        let e = elements[index];
        let age = e.age as f32;
        match e.alphabet {
            'B' => {
                //let last_positions_len = positions.len() as u32;
                let next_pos = pos + rotation_matrix * Vec3::new(0.0, render_option.unit_length, 0.0);
                let radius = 0.5 * render_option.unit_width * (1.0 + render_option.width_growth_factor * age);
                let height = render_option.unit_length;
                let center = (pos + next_pos) * 0.5;
                let p1: [f32; 3] = [-radius, -height*0.7, 0.0];
                let p2: [f32; 3] = [radius, -height*0.7, 0.0];
                let p3: [f32; 3] = [radius, height*0.7, 0.0];
                let p4: [f32; 3] = [-radius, height*0.7, 0.0];
                /*let u = rotation_matrix * Vec3::new(0.0, render_option.unit_length, 0.0);
                let next_pos = u + pos;
                let v = Vec2::new(-u.y, u.x).normalize().extend(0.0);
                let w = v * 0.5 * render_option.unit_width * (1.0 + 0.1 * age);

                let p1: [f32; 3] = (pos - w).into();
                let p2: [f32; 3] = (next_pos - w).into();
                let p3: [f32; 3] = (next_pos + w).into();
                let p4: [f32; 3] = (pos + w).into();*/
                let normal: [f32; 3] = Vec3::new(0.0, 0.0, 1.0).into();
                let uv: [f32; 2] = Vec2::new(0.5, 0.5).into();

                /*
                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
                mesh.set_indices(Some(Indices::U32(vec![0,1,2, 2,3,0])));
                mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vec![p1, p2, p3, p4]);
                mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, vec![normal, normal, normal, normal]);
                mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, vec![uv, uv, uv, uv]);
                */
                let mesh = Mesh::from(shape::Capsule {
                    radius,
                    depth: height*0.5,
                    ..Default::default()
                });
                let transform = Transform {
                    translation: center,
                    rotation: Quat::from_mat3(&rotation_matrix),
                    ..Default::default()
                };
                let component = PlantBodyData {
                    mesh,
                    material: StandardMaterial {
                        base_color: render_option.branch_color.into(),
                        metallic: 0.0,
                        perceptual_roughness: 0.5,
                        ..Default::default()
                    },
                    transform,
                    collider: ColliderBundle {
                        /*shape: ColliderShape::trimesh(vec![p1.into(), p2.into(), p3.into(), p4.into()], vec![[0,1,2],[2,3,0]]).into(),
                        position: [0.0, 0.0, 0.0].into(),*/
                        shape: ColliderShape::capsule([0.0, height * 0.5 - radius, 0.0].into(), [0.0, -height*0.5 + radius, 0.0].into(), radius).into(),
                        position: ColliderPositionComponent(ColliderPosition(
                            Isometry {
                                rotation: Rotation3::from_matrix_unchecked(rotation_matrix.into()).into(),
                                translation: center.into(),
                                ..Default::default()
                            }
                        )),
                        ..Default::default()
                    },
                    rigid_body: RigidBodyBundle {
                        //body_type: RigidBodyType::Static.into(),
                        /*position: 
                            Isometry {
                                rotation: Rotation3::from_matrix_unchecked(rotation_matrix.into()).into(),
                                translation: Translation3::new(u.x*0.5, u.y*0.5, u.z*0.5),
                                ..Default::default()
                            }.into(),*/
                        ..Default::default()
                    },
                };
                let joint = FixedJoint::new();
                components.bodies.push(component);
                components.joints.push(PlantJointData {
                    joint,
                    parent_index: last_body_index,
                    child_index: components.bodies.len() - 1,
                });
                last_body_index = components.bodies.len() - 1;
                pos = next_pos;
            }
            '+' => {
                rotation_matrix = rotation_matrix * Mat3::from_axis_angle(Vec3::Z, render_option.unit_angle);
            }
            '-' => {
                rotation_matrix = rotation_matrix * Mat3::from_axis_angle(Vec3::Z, -render_option.unit_angle);
            }
            '|' => {
                rotation_matrix = rotation_matrix * Mat3::from_axis_angle(Vec3::Y, std::f32::consts::PI);
            }
            '[' => {
                if let Some(new_index) = _render_component_datas(components, elements, index + 1, pos, rotation_matrix, render_option) {
                    index = new_index;
                }
            }
            ']' => {
                returning_index = Some(index);
                break;
            }
            _ => {
            }
        }
        index += 1;
    }
    returning_index
}

pub fn render_component_datas(elements: &[LElement], render_option: &PlantShape) -> PlantComponentDatas {
    let mut components = PlantComponentDatas {
        bodies: Vec::new(),
        joints: Vec::new(),
    };
    _render_component_datas(&mut components, elements, 0, Vec3::new(0.0, 0.0, 0.0), Mat3::IDENTITY, render_option);
    components 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_renders_components() {
        let e = vec![
            LElement { alphabet: 'B', age: 1 }, 
            LElement { alphabet: '[', age: 1 }, 
            LElement { alphabet: '+', age: 1 }, 
            LElement { alphabet: 'B', age: 1 }, 
            LElement { alphabet: ']', age: 0 }, 
            LElement { alphabet: '[', age: 1 }, 
            LElement { alphabet: '|', age: 0 }, 
            LElement { alphabet: '+', age: 1 }, 
            LElement { alphabet: 'B', age: 1 }, 
            LElement { alphabet: ']', age: 0 },
            LElement { alphabet: 'B', age: 0 }
        ];
        let meshdatas = render_meshdatas(e.as_slice(), &PlantShape {
            unit_length: 1.0,
            unit_width: 1.0,
            unit_angle: std::f32::consts::FRAC_PI_2,
        });
        assert_eq!(meshdatas, Vec::new());
    }
}
