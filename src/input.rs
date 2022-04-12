use wgpu::Color;

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    pub color: Color,
    pub name: String,
    position: [f32; 3],
}

impl Node {
    pub fn new(name: impl AsRef<str>, color: impl Into<Color>) -> Self {
        Self {
            color: color.into(),
            name: name.as_ref().to_string(),
            position: [0.0, 0.0, 0.0],
        }
    }
}