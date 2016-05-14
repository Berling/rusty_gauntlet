extern crate glium;
extern crate cgmath;

use std::convert;
use std::io::Cursor;
use std::string::String;
use std::path::Path;
use self::cgmath::Matrix4;
use self::cgmath::Vector2;
use self::cgmath::Vector3;
use self::cgmath::Vector4;
use self::cgmath::VectorSpace;
use self::glium::uniforms;
use self::glium::Surface;
use self::glium::texture::texture2d::Texture2d;
use image;
use rendering::vertex;

pub struct Sprite {
    vbo: glium::vertex::VertexBuffer<vertex::Vertex>,
    position: Vector2<f32>,
    texture: Texture2d,
}

impl Sprite {
    pub fn draw(&self, surface: &mut glium::Frame, program: &glium::Program, projection: Matrix4<f32>, view: Matrix4<f32>) {
        let ibo = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let model = Matrix4::<f32>::from_translation(
            Vector3::<f32>{ x: self.position.x, y: self.position.y, z: -1.0 }
        );
        let uniforms = uniform!{
            projection: Into::<[[f32; 4]; 4]>::into(projection),
            view: Into::<[[f32; 4]; 4]>::into(view),
            model: Into::<[[f32; 4]; 4]>::into(model),
            sprite: &self.texture
        };
        surface.draw(
            &self.vbo,
            &ibo,
            program,
            &uniforms,
            &Default::default()
        ).unwrap();
    }

    pub fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
    }
}

pub struct SpriteBuilder {
    half_extend: Vector2<f32>,
    position: Vector2<f32>,
    texture_name: String,
}

impl SpriteBuilder {
    pub fn new() -> SpriteBuilder {
        SpriteBuilder {
            half_extend: Vector2{ x: 0.0, y: 0.0 },
            position: Vector2{ x: 0.0, y: 0.0 },
            texture_name: String::from("")
        }
    }

    pub fn half_extend(&mut self, half_extend: Vector2<f32>) -> &mut SpriteBuilder {
        self.half_extend = half_extend;
        self
    }

    pub fn position(&mut self, position: Vector2<f32>) -> &mut SpriteBuilder {
        self.position = position;
        self
    }

    pub fn texture_name(&mut self, texture_name: &str) -> &mut SpriteBuilder {
        self.texture_name = String::from(texture_name);
        self
    }

    pub fn finalize<F: glium::backend::Facade>(&mut self, facade: &F) -> Sprite {
        let vertices = vec![
            vertex::VertexBuilder::new()
                .position(Vector2{ x: self.half_extend.x, y: self.half_extend.y })
                .texcoord(Vector2{ x: 1.0, y: 0.0 })
                .color(Vector4{ x: 1.0, y: 0.0, z: 0.0, w: 0.0 })
                .finalize(),
            vertex::VertexBuilder::new()
                .position(Vector2{ x: -self.half_extend.x, y: self.half_extend.y })
                .texcoord(Vector2{ x: 0.0, y: 0.0 })
                .color(Vector4{ x: 1.0, y: 0.0, z: 0.0, w: 0.0 })
                .finalize(),
            vertex::VertexBuilder::new()
                .position(Vector2{ x: -self.half_extend.x, y: -self.half_extend.y })
                .texcoord(Vector2{ x: 0.0, y: 1.0 })
                .color(Vector4{ x: 1.0, y: 0.0, z: 0.0, w: 0.0 })
                .finalize(),
            vertex::VertexBuilder::new()
                .position(Vector2{ x: -self.half_extend.x, y: -self.half_extend.y })
                .texcoord(Vector2{ x: 0.0, y: 1.0 })
                .color(Vector4{ x: 1.0, y: 0.0, z: 0.0, w: 0.0 })
                .finalize(),
            vertex::VertexBuilder::new()
                .position(Vector2{ x: self.half_extend.x, y: -self.half_extend.y })
                .texcoord(Vector2{ x: 1.0, y: 1.0 })
                .color(Vector4{ x: 1.0, y: 0.0, z: 0.0, w: 0.0 })
                .finalize(),
            vertex::VertexBuilder::new()
                .position(Vector2{ x: self.half_extend.x, y: self.half_extend.y })
                .texcoord(Vector2{ x: 1.0, y: 0.0 })
                .color(Vector4{ x: 1.0, y: 0.0, z: 0.0, w: 0.0 })
                .finalize(),
        ];
        let image = image::open(&Path::new(self.texture_name.as_str())).unwrap().to_rgba();
        let dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), dimensions);
        Sprite {
            vbo: glium::VertexBuffer::<vertex::Vertex>::new(
                facade,
                &vertices
            ).unwrap(),
            position: self.position,
            texture: Texture2d::new(facade, image).unwrap()
        }
    }
}
