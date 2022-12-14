use helpers::fen::{parse_fen, STARTING_POSITION};
use macroquad::prelude::*;
use macroquad::window::Conf;
use objects::tile::*;
use objects::piece::*;

mod objects;
mod helpers;

const BOARD_SIZE: i32 = 640;
const TILE_SIZE: f32 = BOARD_SIZE as f32 / 8f32;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty chess".to_owned(),
        window_height: BOARD_SIZE,
        window_width: BOARD_SIZE,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut tiles = Vec::new();
    init_board(&mut tiles).await;

    parse_fen(STARTING_POSITION);

    let piece = Piece::new(vec2(0f32, 0f32), PieceType::Queen, PieceColor::White, TILE_SIZE);

    loop {
        clear_background(WHITE);

        for tile in tiles.iter() {
            tile.draw();
        }

        piece.draw();

        next_frame().await
    }
}
