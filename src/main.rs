use macroquad::prelude::*;
use macroquad::window::Conf;
use objects::tile::*;


mod objects;

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

async fn init_board(tiles: &mut Vec<Tile>){
    for i in 0..32 {
        let j = i as f32;

        let mut x = ((j * 2f32) % 8f32) * TILE_SIZE;
        let y = i / 4;

        if y % 2 == 0 {
            x += TILE_SIZE;
        }

        tiles.push(
            Tile::new(vec2( x, y as f32 * TILE_SIZE), 
            vec2(TILE_SIZE, TILE_SIZE), BLACK)
        );
}}

#[macroquad::main(window_conf)]
async fn main() {
    let mut tiles = Vec::new();
    init_board(&mut tiles).await;
    
    loop {
        clear_background(WHITE);

        for tile in tiles.iter() {
            tile.draw();
        }

        next_frame().await
    }
}
