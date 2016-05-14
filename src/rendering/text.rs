extern crate cgmath;

use glium_text::TextSystem;
use glium_text::FontTexture;
use glium_text::TextDisplay;
use glium_text;
use glium::Frame;
use std::path::Path;
use std::fs::File;
use glium::backend::Facade;
use self::cgmath::Matrix4;
use self::cgmath::prelude::SquareMatrix;
use self::cgmath::Vector2;

pub struct TextRenderer {
    system: TextSystem,
    font: FontTexture,
    font_size: f32
}

impl TextRenderer {
    pub fn new<F: Facade>(facade: &F, font_name: &str, font_size: u32) -> TextRenderer {
        let system = TextSystem::new(facade);
        let font = FontTexture::new(
            facade,
            File::open(&Path::new(font_name)).unwrap(),
            font_size
        ).unwrap();
        TextRenderer {
            system: system,
            font: font,
            font_size: font_size as f32
        }
    }

    pub fn draw(&mut self, surface: &mut Frame, text: &str, position: Vector2<f32>) {
        let text_display = TextDisplay::new(&self.system, &self.font, text);
        let scale = self.font_size / 600.0;
        let matrix = [
            [scale, 0.0, 0.0, 0.0],
            [0.0, scale, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [2.0 * (position.x / 800.0) - 1.0, -2.0 * (position.y / 600.0) + 1.0, 0.0, 1.0]
        ];
        glium_text::draw(&text_display, &self.system, surface, matrix, (0.0, 0.0, 0.0, 1.0));
    }
}
