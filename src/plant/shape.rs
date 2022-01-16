#[derive(Clone, Debug)]
pub struct PlantShape {
    pub unit_length: f32,
    pub unit_angle: f32,
    pub unit_width: f32,
    pub width_growth_factor: f32,
    pub branch_color: [f32; 4],
}
