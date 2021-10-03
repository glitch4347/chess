use macroquad::prelude::*;

use chess::game::Game;

#[macroquad::main("Chess")]
async fn main() -> anyhow::Result<()> {
    
    // {
    //     // LOADING
    //     clear_background(WHITE);
    //     let text = "Loading...";
    //     let font_size = 30.;
    //     let text_size = measure_text(text, None, font_size as _, 1.0);

    //     draw_text(
    //         text,
    //         screen_width() / 2. - text_size.width / 2.,
    //         screen_height() / 2. - text_size.height,
    //         font_size,
    //         DARKGRAY,
    //     );
    //     next_frame().await
    // }

    
    let mut game = Game::new().await?;
    
    let mut game_over = false;

    loop {
        if !game_over {
            game.handle_input();
            game.render();
        } else {
            clear_background(WHITE);
            if is_key_down(KeyCode::Enter) {
                game = Game::new().await?;
            }
        }
        next_frame().await
    }

}
