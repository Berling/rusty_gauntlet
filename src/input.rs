extern crate glium;

pub mod input {

    use input::glium::glutin::Event;

    pub struct Input;

    pub enum Direction {
        UP,
        DOWN,
        LEFT,
        RIGHT,
    }
    
    impl Input {

        pub fn new() -> Input {
            Input {}
        }
        
        pub fn process_input(&self, event:Event) -> bool {
            use input::glium::glutin::Event::KeyboardInput as Keyboard;
            use input::glium::glutin::{ElementState as State, VirtualKeyCode as KeyCode};
            match event {
                Event::Closed => true,
                Keyboard(State::Pressed, _, Some(KeyCode::Escape)) => true,
                Keyboard(State::Pressed, _, Some(KeyCode::Up)) => self.move_dir(Direction::UP),
                Keyboard(State::Pressed, _, Some(KeyCode::Down)) => self.move_dir(Direction::DOWN),
                Keyboard(State::Pressed, _, Some(KeyCode::Left)) => self.move_dir(Direction::LEFT),
                Keyboard(State::Pressed, _, Some(KeyCode::Right)) => self.move_dir(Direction::RIGHT),
                Keyboard(State::Pressed, _, Some(KeyCode::Return)) => false,
                _ => false,
            }
        }

        pub fn move_dir(&self, dir:Direction) -> bool{

            false
        }
    }
}