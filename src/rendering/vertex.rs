extern crate cgmath;
extern crate glium;

use std::str;
use std::borrow::Cow;
use std::mem::size_of;

use self::cgmath::Vector2;
use self::cgmath::Vector4;
use self::glium::vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Vector2<f32>,
    pub texcoord: Vector2<f32>,
    pub color: Vector4<f32>,
}

pub struct VertexBuilder {
    pub position: Vector2<f32>,
    pub texcoord: Vector2<f32>,
    pub color: Vector4<f32>,
}

impl VertexBuilder {
    pub fn new() -> VertexBuilder {
        VertexBuilder {
            position: Vector2 { x: 0.0, y: 0.0 },
            texcoord: Vector2 { x: 0.0, y: 0.0 },
            color: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
        }
    }

    pub fn position(&mut self, position: Vector2<f32>) -> &mut VertexBuilder {
        self.position = position;
        self
    }

    pub fn texcoord(&mut self, texcoord: Vector2<f32>) -> &mut VertexBuilder {
        self.texcoord = texcoord;
        self
    }

    pub fn color(&mut self, color: Vector4<f32>) -> &mut VertexBuilder {
        self.color = color;
        self
    }

    pub fn finalize(&self) -> Vertex {
        Vertex { position: self.position, texcoord: self.texcoord, color: self.color }
    }
}

impl vertex::Vertex for Vertex {
    fn build_bindings() -> vertex::VertexFormat {
        Cow::Owned::<'static, [(Cow<'static, str>, usize, vertex::AttributeType)]>(
            vec![
                (Cow::Owned(String::from("_position")), 0, vertex::AttributeType::F32F32),
                (Cow::Owned(String::from("_texcoord")), size_of::<Vector2<f32>>(), vertex::AttributeType::F32F32),
                (Cow::Owned(String::from("_color")), size_of::<Vector2<f32>>() + size_of::<Vector2<f32>>(), vertex::AttributeType::F32F32F32F32),
            ]
        )
    }
}
