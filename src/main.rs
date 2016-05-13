#[macro_use]
extern crate glium;
extern crate rusty_gauntlet;
use rusty_gauntlet::level::*;
use std::path::Path;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title(format!("Rusty Gauntlet™"))
        .build_glium()
        .unwrap();

    let mut my_level = level::Level::new(Path::new("test_level.map"));
    let mut player_pos = (2,2);
    player_pos = my_level.interact(player_pos,level::Direction::Right);
    player_pos = my_level.interact(player_pos,level::Direction::Right);
    player_pos = my_level.interact(player_pos,level::Direction::Down);
    my_level.interact(player_pos,level::Direction::Down);
    my_level.debug_print();

    loop {
        let mut target = display.draw();
        target.clear_color(0.5, 0.6, 0.9, 1.0);
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
