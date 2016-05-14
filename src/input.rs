extern crate glium;

use self::glium::glutin::Event;
use level::Direction;

pub struct Input;

impl Input {

    pub fn new() -> Input {
        Input {}
    }
    
    pub fn process_input(&self, event:Event) -> Option<bool> {
        use self::glium::glutin::Event::KeyboardInput as Keyboard;
        use self::glium::glutin::{ElementState as State, VirtualKeyCode as KeyCode};
        match event {
            Event::Closed => None,
            Keyboard(State::Pressed, _, Some(KeyCode::Escape)) => None,
            Keyboard(State::Pressed, _, Some(KeyCode::Up)) => self.move_dir(Direction::Up),
            Keyboard(State::Pressed, _, Some(KeyCode::Down)) => self.move_dir(Direction::Down),
            Keyboard(State::Pressed, _, Some(KeyCode::Left)) => self.move_dir(Direction::Left),
            Keyboard(State::Pressed, _, Some(KeyCode::Right)) => self.move_dir(Direction::Right),
            Keyboard(State::Pressed, _, Some(KeyCode::Return)) => Some(true),
            _ => Some(true),
        }
    }

    pub fn move_dir(&self, dir:Direction) -> Option<bool> {
        match dir {
            Direction::Up => println!("Up"),
            _ => (),
        }
        Some(false)
    }
}