#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate rusty_gauntlet;

use cgmath::Matrix4;
use cgmath::Vector2;
use cgmath::prelude::SquareMatrix;
use rusty_gauntlet::rendering::sprite;
use rusty_gauntlet::level::*;
use rusty_gauntlet::input::*;
use std::path::Path;
use rusty_gauntlet::rendering::text::TextRenderer;
use std::string::String;
use cgmath::Vector4;


static mut player_alive: bool = true;

fn on_damaged(_: &Entity, _: &Entity) {
    println!(" > The dragon bites!");
}
fn on_attacked(_: &Entity, _: &Entity) {
    println!(" > You poked the dragon with a stick!");
}
fn on_killed(_: &Entity) {
    println!(" > You died!");
    unsafe{ player_alive = false; }
}
fn on_collected(_: &Entity) {
    println!(" > You found a coin!");
}

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title(format!("Rusty Gauntlet™"))
        .build_glium()
        .unwrap();

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

    const SCREEN_WIDTH: f32 = 800.0;
    const SCREEN_HEIGHT: f32 = 600.0;
    let projection = cgmath::ortho::<f32>(0.0, SCREEN_WIDTH, SCREEN_HEIGHT, 0.0, 0.1, 10.0);
    let view = Matrix4::<f32>::identity();

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
        .unwrap();

    const TILE_SIZE: f32 = 32f32;
    let mut sprite_wall = sprite::SpriteBuilder::new()
        .half_extend(Vector2{ x: TILE_SIZE, y: TILE_SIZE })
        .texture_name("sprite_wall.png")
        .finalize(&display);
    let mut sprite_floor = sprite::SpriteBuilder::new()
        .half_extend(Vector2{ x: TILE_SIZE, y: TILE_SIZE })
        .texture_name("sprite_floor.png")
        .finalize(&display);
    let mut sprite_player = sprite::SpriteBuilder::new()
        .half_extend(Vector2{ x: TILE_SIZE, y: TILE_SIZE })
        .texture_name("sprite_player.png")
        .finalize(&display);
    let mut sprite_dragon = sprite::SpriteBuilder::new()
        .half_extend(Vector2{ x: TILE_SIZE, y: TILE_SIZE })
        .texture_name("sprite_dragon.png")
        .finalize(&display);
    let mut sprite_treasure = sprite::SpriteBuilder::new()
        .half_extend(Vector2{ x: TILE_SIZE, y: TILE_SIZE })
        .texture_name("sprite_treasure.png")
        .finalize(&display);

    let mut my_level = Level::new(Path::new("test_level.map"));
    my_level.on_player_damaged = Some(on_damaged);
    my_level.on_player_attacked = Some(on_attacked);
    my_level.on_player_killed = Some(on_killed);
    my_level.on_player_collected = Some(on_collected);

    let mut player_pos = my_level.get_player_pos().unwrap();
    /*
    let (pscore, php) = match my_level.get_entity(player_pos) {
        Some(Entity::Player{score,hp,..}) => (score,hp),
        _ => (0,0)
    };
    println!("Player score: {}\nPlayer HP: {}", pscore, php);
*/
    let mut input = Input::new(player_pos);

    let mut text_renderer = TextRenderer::new(
        &display,
        "font.otf",
        45
    );

    let mut dead_text_renderer = TextRenderer::new(
        &display,
        "font.otf",
        70
    );

    loop {
        let mut target = display.draw();
        target.clear_color(0.24, 0.24, 0.24, 1.0);

        let (px,py) = input.player_pos;
        let offset_x = SCREEN_WIDTH/2.0 - px as f32 *TILE_SIZE*2f32;
        let offset_y = SCREEN_HEIGHT/2.0 - py as f32 *TILE_SIZE*2f32;
        my_level.foreach(|x,y,tile| {
            let pos = Vector2{ x: offset_x + TILE_SIZE*2f32*x as f32, y: offset_y + TILE_SIZE*2f32*y as f32 };

            match tile.entity {
                Some(Entity::Player{..}) => {
                    sprite_player.set_position(pos);
                    sprite_player.draw(&mut target, &program, projection, view);
                },
                Some(Entity::Dragon{..}) =>  {
                    sprite_dragon.set_position(pos);
                    sprite_dragon.draw(&mut target, &program, projection, view);
                },
                Some(Entity::Treasure) => {
                    sprite_treasure.set_position(pos);
                    sprite_treasure.draw(&mut target, &program, projection, view);
                },
                None => {
                    match tile.tile_type {
                        TileType::Floor => {
                            sprite_floor.set_position(pos);
                            sprite_floor.draw(&mut target, &program, projection, view);
                        },
                        TileType::Wall => {
                            sprite_wall.set_position(pos);
                            sprite_wall.draw(&mut target, &program, projection, view);
                        },
                    }
                }
            };
        });

        let (pscore, php) = match my_level.get_entity(input.player_pos) {
            Some(Entity::Player{score,hp,..}) => (score,hp),
            _ => (0,0)
        };

        let coin_label = "Coin: ".to_string();
        let coin = pscore.to_string();
        let hp_label = "      Health; ".to_string();
        let hp = php.to_string();
        let final_label = coin_label + coin.as_str() + hp_label.as_str() + hp.as_str();
        text_renderer.draw(&mut target, final_label.as_str(), Vector2::<f32>{ x: 5.0, y: 590.0 }, Vector4::<f32>{ x: 0.2, y: 0.9, z: 0.0, w: 1.0});

        let alive = unsafe{ player_alive};
        if !alive {
            dead_text_renderer.draw(&mut target, "YOU'RE DEAD!   D   E   D!!!   DEAD!", Vector2::<f32>{ x: 27.0, y: 300.0 }, Vector4::<f32>{ x: 0.8, y: 0.0, z: 0.0, w: 1.0});
        }

        target.finish().unwrap();

        if alive {
            //handle events
            let mut player_input = Some(true);
            //loop till player is moved
            //none means ends game
            //false means advance game
            //true means wait for next input
            while player_input.unwrap() {
                for ev in display.poll_events() {
                    player_input = input.process_input(ev, &mut my_level);
                }
                if player_input.is_none() {
                    return;
                }
            }
        }
    }
}
