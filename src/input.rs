extern crate glium;

pub mod input {

    use input::glium::glutin::Event;

    pub struct Input;
    
    impl Input {

        pub fn new() -> Input {
            Input {}
        }
        
        pub fn process_input(&self, event:Event) -> bool {

            use input::glium::glutin::{ElementState, VirtualKeyCode as KeyCode};
            match event {
                Event::Closed => true,
                Event::KeyboardInput(ElementState::Pressed, _, Some(KeyCode::Escape)) => true,
                
                _ => false,
            }
        }


    }
}