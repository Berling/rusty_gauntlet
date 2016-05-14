extern crate glium;

use self::glium::glutin::Event;
use level::{Direction, Level};

pub struct Input {
    pub player_pos: (i32, i32),
}

impl Input {

    pub fn new(player_pos: (i32, i32)) -> Input {
        Input {player_pos:player_pos}
    }

    pub fn process_input(&mut self, event:Event, level:&mut Level) -> Option<bool> {
        use self::glium::glutin::Event::KeyboardInput as Keyboard;
        use self::glium::glutin::{ElementState as State, VirtualKeyCode as KeyCode};
        match event {
            Event::Closed => None,
            Keyboard(State::Pressed, _, Some(KeyCode::Escape)) => None,
            Keyboard(State::Pressed, _, Some(KeyCode::Up)) => self.move_dir(Direction::Up, level),
            Keyboard(State::Pressed, _, Some(KeyCode::Down)) => self.move_dir(Direction::Down, level),
            Keyboard(State::Pressed, _, Some(KeyCode::Left)) => self.move_dir(Direction::Left, level),
            Keyboard(State::Pressed, _, Some(KeyCode::Right)) => self.move_dir(Direction::Right, level),
            Keyboard(State::Pressed, _, Some(KeyCode::Return)) => Some(true),
            _ => Some(true),
        }
    }

    pub fn move_dir(&mut self, dir:Direction, mut level:&mut Level) -> Option<bool> {
        self.player_pos = level.interact(self.player_pos, dir);

        //advance ai
        use ai::ai_step;
        ai_step(&mut level, self.player_pos);
        Some(false)
    }
}
