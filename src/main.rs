#[macro_use]
extern crate glium;
extern crate rusty_gauntlet;
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
