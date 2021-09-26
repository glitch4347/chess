use macroquad::prelude::*;
use crate::field::{self, Field};
use crate::state::State;

pub struct Game {
    field: Field,
    pub state: State,
    mouse_down: bool,
}

impl Game {
    pub async fn new() -> anyhow::Result<Game> {
        return Ok(Game {
            field: Field::new().await?,
            state: State::new(),
            mouse_down: false
        });
    }

    pub fn handle_input(&mut self) {
        
        if is_mouse_button_down(MouseButton::Left) {
            self.mouse_down = true;
        } else {
            if self.mouse_down {
                self.on_click();
                self.mouse_down = false;
            }
        }
    }

    pub fn render(&self) {
        self.field.render(&self);
    }

    pub fn width(&self) -> f32 {
        return screen_width();
    }
    pub fn height(&self) -> f32 {
        return screen_height();
    }

    pub fn on_click(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        println!("{} {}", mouse_x, mouse_y);
    }
}

