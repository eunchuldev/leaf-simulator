use super::LElement;
use super::PlantShape;
//use bevy_prototype_lyon::geometry::Geometry;
use bevy::{
    math::{Vec3, Vec2, Mat3},
    render::{
        mesh::{Indices, Mesh},
        render_resource::PrimitiveTopology,
    },
};
//use bevy::prelude::*;
//use bevy_rapier3d::prelude::*;

/*use lyon_tessellation::{
    math::{Point, Rect, Size},
    path::{path::Builder, traits::PathBuilder, Path, Winding},
};*/
fn normal(p1: Vec3, p2: Vec3, p3: Vec3) -> Vec3 {
    let dir = (p2 - p1).cross(p3 - p1);
    dir.normalize()
}

#[derive(Debug, PartialEq)]
pub struct MeshData {
    indices: Vec<u32>, 
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
}

impl From<MeshData> for Mesh {
    fn from(f: MeshData) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(f.indices)));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, f.positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, f.normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, f.uvs);
        mesh
    }
}

fn _render_meshdatas(
    meshdatas: &mut Vec<MeshData>,
    elements: &[LElement], 
    mut index: usize, 
    mut pos: Vec3, 
    mut rotation_matrix: Mat3, 
    render_option: &PlantShape,
) -> Option<usize> {
    let mut returning_index = None; 
    let mut positions = Vec::<[f32; 3]>::new();
    let mut normals = Vec::<[f32; 3]>::new();
    let mut indices = Vec::<u32>::new();
    let mut uvs = Vec::<[f32; 2]>::new();
    while index < elements.len() {
        let e = elements[index];
        let age = e.age as f32;
        match e.alphabet {
            'B' => {
                let last_positions_len = positions.len() as u32;
                let u = rotation_matrix * Vec3::new(0.0, render_option.unit_length, 0.0);
                let next_pos = u + pos;
                let v = Vec2::new(-u.y, u.x).normalize().extend(0.0);
                let w = v * 0.5 * render_option.unit_width * (1.0 + 0.1 * age);
                let p1 = pos - w;
                let p2 = next_pos - w;
                let p3 = next_pos + w;
                let p4 = pos + w;
                println!("{:?}, {:?}, {:?}, {:?}", p1, p2, p3, p4);
                //let p1 = rotation_matrix * Vec3::new(w * (-0.5 - 0.1 * age), 0.0, 0.0) + pos;
                //let p2 = rotation_matrix * Vec3::new(w * (0.5 + 0.1 * age), 0.0, 0.0) + pos;
                //let p3 = rotation_matrix * Vec3::new(w * (0.5 + 0.1 * age), l, 0.0) + pos;
                //let p4 = rotation_matrix * Vec3::new(w * (-0.5 - 0.1 * age), l, 0.0) + pos;
                /*if normal(p1, p2, p3).z >= 0.0 {
                    positions.push(p1.into());
                    positions.push(p2.into());
                    positions.push(p3.into()); 
                    positions.push(p4.into());
                } else {
                }*/
                positions.push(p1.into());
                positions.push(p2.into());
                positions.push(p3.into()); 
                positions.push(p4.into());
                normals.push(Vec3::new(0.0, 0.0, 1.0).into());
                normals.push(Vec3::new(0.0, 0.0, 1.0).into());
                normals.push(Vec3::new(0.0, 0.0, 1.0).into());
                normals.push(Vec3::new(0.0, 0.0, 1.0).into());
                uvs.push([0.5, 0.5]);
                uvs.push([0.5, 0.5]);
                uvs.push([0.5, 0.5]);
                uvs.push([0.5, 0.5]);
                indices.push(last_positions_len + 0);
                indices.push(last_positions_len + 1);
                indices.push(last_positions_len + 2);
                indices.push(last_positions_len + 2);
                indices.push(last_positions_len + 3);
                indices.push(last_positions_len + 0);
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
                if let Some(new_index) = _render_meshdatas(meshdatas, elements, index + 1, pos, rotation_matrix, render_option) {
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
    meshdatas.push(MeshData {
        indices, 
        positions, 
        normals, 
        uvs
    });
    returning_index
}

pub fn render_meshdatas(elements: &[LElement], render_option: &PlantShape) -> Vec<MeshData> {
    let mut meshdatas = Vec::new();
    _render_meshdatas(&mut meshdatas, elements, 0, Vec3::new(0.0, 0.0, 0.0), Mat3::IDENTITY, render_option);
    meshdatas
}

pub fn render_meshes(elements: &[LElement], render_option: &PlantShape) -> Vec<Mesh> {
    render_meshdatas(elements, render_option).into_iter().map(Mesh::from).collect()
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
