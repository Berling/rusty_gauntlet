#[macro_use]
extern crate glium;
extern crate rusty_gauntlet;
use rusty_gauntlet::level::*;
use rusty_gauntlet::input::*;
use std::path::Path;


fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title(format!("Rusty Gauntlet™"))
        .build_glium()
        .unwrap();

    let my_level = level::Level::new(Path::new("test_level.map"));
    my_level.debug_print();

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
