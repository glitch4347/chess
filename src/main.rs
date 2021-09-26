use macroquad::prelude::*;

use chess::game::Game;

#[macroquad::main("Chess")]
async fn main() {
    let mut game = Game::new().await;
    let mut game_over = false;

    loop {
        if !game_over {
            game.handle_keys();
            game.render();
        } else {
            clear_background(WHITE);
            if is_key_down(KeyCode::Enter) {
                game = Game::new().await;
            }
        }
        next_frame().await
    }

}
