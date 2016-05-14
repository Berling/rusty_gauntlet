#[macro_use]
extern crate glium;
extern crate rusty_gauntlet;
use rusty_gauntlet::level::*;
use rusty_gauntlet::ai::*;
use rusty_gauntlet::input::*;
use std::path::Path;


fn on_damaged(_: &Entity, _: &Entity) {
    println!(" > The dragon bites!");
}
fn on_attacked(_: &Entity, _: &Entity) {
    println!(" > You poked the dragon with a stick!");
}
fn on_killed(_: &Entity) {
    println!(" > You died!");
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

    let mut my_level = Level::new(Path::new("test_level.map"));
    my_level.on_player_damaged = Some(on_damaged);
    my_level.on_player_attacked = Some(on_attacked);
    my_level.on_player_killed = Some(on_killed);
    my_level.on_player_collected = Some(on_collected);

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
