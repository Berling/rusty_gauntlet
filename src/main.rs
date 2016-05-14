#[macro_use]
extern crate glium;
extern crate rusty_gauntlet;
use rusty_gauntlet::level::*;
use rusty_gauntlet::input::*;
use rusty_gauntlet::ai::*;
use std::path::Path;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title(format!("Rusty Gauntlet™"))
        .build_glium()
        .unwrap();

    let mut my_level = Level::new(Path::new("test_level.map"));
    let mut player_pos = (2,2);
    player_pos = my_level.interact(player_pos, Direction::Right);
    player_pos = my_level.interact(player_pos, Direction::Right);
    player_pos = my_level.interact(player_pos, Direction::Right);
    player_pos = my_level.interact(player_pos, Direction::Down);
    player_pos = my_level.interact(player_pos, Direction::Down);
    let player_score = match my_level.get_entity(player_pos) {
        Some(Entity::Player{score,..}) => score,
        _ => 0
    };
    println!("Player score: {}", player_score);

    loop {
        let mut target = display.draw();
        target.clear_color(0.5, 0.6, 0.9, 1.0);
        target.finish().unwrap();

        //handle events
        let mut input = Input::new(player_pos);
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
