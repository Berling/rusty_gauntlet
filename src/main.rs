#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate rusty_gauntlet;

use cgmath::Matrix4;
use cgmath::Vector2;
use cgmath::Vector4;
use cgmath::prelude::SquareMatrix;
use rusty_gauntlet::rendering::sprite;
use rusty_gauntlet::rendering::vertex;
use rusty_gauntlet::level::*;
use rusty_gauntlet::ai::*;
use rusty_gauntlet::input::*;
use std::path::Path;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title(format!("Rusty Gauntlet™"))
        .build_glium()
        .unwrap();

    let vertex1 = vertex::VertexBuilder::new()
        .position(Vector2{ x: -0.5, y: -0.5 })
        .color(Vector4{ x: 1.0, y: 0.0, z: 0.0, w: 0.0 })
        .finalize();
    let vertex2 = vertex::VertexBuilder::new()
        .position(Vector2{ x: 0.0, y: 0.5 })
        .color(Vector4{ x: 0.0, y: 1.0, z: 0.0, w: 0.0 })
        .finalize();
    let vertex3 = vertex::VertexBuilder::new()
        .position(Vector2{ x: 0.5, y: -0.25 })
        .color(Vector4{ x: 0.0, y: 0.0, z: 1.0, w: 0.0 })
        .finalize();
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_shader_src = r#"
        #version 140

        in vec2 _position;
        in vec2 _texcoord;
        in vec4 _color;

        out vec2 texcoord_;
        out vec4 color_;

        uniform mat4 projection;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            texcoord_ = _texcoord;
            color_ = _color;
            gl_Position = projection * view * model * vec4(_position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 texcoord_;
        in vec4 color_;

        out vec4 color;

        uniform sampler2D sprite;

        void main() {
            color = texture(sprite, texcoord_);
        }
    "#;

    let projection = cgmath::ortho::<f32>(0.0, 800.0, 600.0, 0.0, 0.1, 10.0);
    let view = Matrix4::<f32>::identity();

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
        .unwrap();

    let mut test = sprite::SpriteBuilder::new()
        .half_extend(Vector2{ x: 200.0, y: 200.0 })
        .texture_name("test.png")
        .finalize(&display);

    test.set_position(Vector2{ x: 400.0, y: 300.0 });

    let mut my_level = Level::new(Path::new("test_level.map"));
    let mut player_pos = my_level.get_player_pos().unwrap();
    ai_step(&mut my_level, player_pos);
    ai_step(&mut my_level, player_pos);
    ai_step(&mut my_level, player_pos);
    player_pos = my_level.interact(player_pos, Direction::Right);
    player_pos = my_level.interact(player_pos, Direction::Right);
    player_pos = my_level.interact(player_pos, Direction::Right);
    player_pos = my_level.interact(player_pos, Direction::Down);
    player_pos = my_level.interact(player_pos, Direction::Down);
    my_level.debug_print();
    let (pscore, php) = match my_level.get_entity(player_pos) {
        Some(Entity::Player{score,hp,..}) => (score,hp),
        _ => (0,0)
    };
    println!("Player score: {}\nPlayer HP: {}", pscore, php);

    loop {
        let mut target = display.draw();
        target.clear_color(0.5, 0.6, 0.9, 1.0);
        test.draw(&mut target, &program, projection, view);
        target.finish().unwrap();

        //handle events
        let input = input::Input::new();
        for ev in display.poll_events() {
        	if input.process_input(ev) {
        		return;
        	}
        }
    }
}
