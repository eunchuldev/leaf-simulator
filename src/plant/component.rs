use crate::lsystem::LElement;
use super::Shape;
use nalgebra::{Point3, Matrix3, Vector3, Rotation3};
use bevy::{
    math::{Vec3, Mat3},
    render::{
        mesh::{Indices, Mesh},
        render_resource::PrimitiveTopology,
    },
};


#[derive(Clone, Debug)]
pub enum Component {
    Branch {
        mesh: Mesh,
    },
    Leaf {
        mesh: Mesh,
    }
}

fn _render_components(components: &mut Vec<Component>, elements: &[LElement], mut index: usize, mut pos: Point3<f32>, mut rotation_matrix: Matrix3<f32>, render_option: &Shape) -> Option<usize> {
    //let left = Vec::new();
    //let right = Vec::new();
    while index < elements.len() {
        let e = elements[index];
        match e.alphabet {
            'B' => {
                let next_pos = rotation_matrix * Point3::new(0.0, render_option.unit_length, 0.0) + pos.coords;
                components.push(Component::Branch {
                    p1: pos,
                    p2: next_pos,
                    width: 1.0 + e.age as f32,
                });
                pos = next_pos;
            }
            '+' => {
                rotation_matrix = rotation_matrix * Rotation3::from_axis_angle(&Vector3::z_axis(), render_option.unit_angle);
            }
            '-' => {
                rotation_matrix = rotation_matrix * Rotation3::from_axis_angle(&Vector3::z_axis(), -render_option.unit_angle);
            }
            '|' => {
                rotation_matrix = rotation_matrix * Rotation3::from_axis_angle(&Vector3::y_axis(), std::f32::consts::PI);
            }
            '[' => {
                if let Some(new_index) = _render_components(components, elements, index + 1, pos, rotation_matrix, render_option) {
                    index = new_index;
                }
            }
            ']' => {
                return Some(index);
            }
            _ => {
            }
        }
        index += 1;
    }
    None
}

pub fn render_components(elements: &[LElement], render_option: &Shape) -> Vec<Component> {
    let mut components = Vec::new();
    _render_components(&mut components, elements, 0, Point3::new(0.0, 0.0, 0.0), Rotation3::identity().into(), render_option);
    components
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
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
        let components = render_components(e.as_slice(), &Shape {
            unit_length: 1.0,
            unit_angle: std::f32::consts::FRAC_PI_2,
        });
        let expected = vec![
            Component::Branch { 
                p1: Point3::new(0.0, 0.0, 0.0),
                p2: Point3::new(0.0, 1.0, 0.0),
                width: 2.0,
            }, 
            Component::Branch {
                p1: Point3::new(0.0, 1.0, 0.0),
                p2: Point3::new(-1.0, 1.0, 0.0),
                width: 2.0,
            }, 
            Component::Branch {
                p1: Point3::new(0.0, 1.0, 0.0),
                p2: Point3::new(1.0, 1.0, 0.0),
                width: 2.0,
            }, 
            Component::Branch { 
                p1: Point3::new(0.0, 1.0, 0.0),
                p2: Point3::new(0.0, 2.0, 0.0),
                width: 1.0,
            }
        ];
        for (a, b) in expected.into_iter().zip(components.into_iter()) {
            match (&a, &b) {
                (
                    Component::Branch{ p1: ap1, p2: ap2, width: awidth },
                    Component::Branch{ p1: bp1, p2: bp2, width: bwidth },
                ) => {
                    assert_relative_eq!(ap1, bp1);
                    assert_relative_eq!(ap2, bp2);
                    assert_relative_eq!(awidth, bwidth);
                }
                (
                    Component::Leaf { polygon: apolygon, },
                    Component::Leaf { polygon: bpolygon, },
                ) => {
                    for (c, d) in apolygon.iter().zip(bpolygon.iter()) {
                        assert_relative_eq!(c, d);
                    }
                }
                _ => {
                    panic!("component type mismatch: {:?}, {:?}", &a, &b);
                }
            }
        }
    }
}
