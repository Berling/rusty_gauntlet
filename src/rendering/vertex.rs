extern crate cgmath;
use self::cgmath::Vector2;
use self::cgmath::Vector4;

pub struct Vertex {
    pub position: Vector2<f32>,
    pub color: Vector4<f32>,
}

pub struct VertexBuilder {
    pub position: Vector2<f32>,
    pub color: Vector4<f32>,
}

impl VertexBuilder {
    pub fn new() -> VertexBuilder {
        VertexBuilder {
            position: Vector2 { x: 0.0, y: 0.0 },
            color: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
        }
    }

    pub fn position(&mut self, position: Vector2<f32>) -> &mut VertexBuilder {
        self.position = position;
        self
    }

    pub fn color(&mut self, color: Vector4<f32>) -> &mut VertexBuilder {
        self.color = color;
        self
    }

    pub fn finalize(&self) -> Vertex {
        Vertex { position: self.position, color: self.color }
    }
}
