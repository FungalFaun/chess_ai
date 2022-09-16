use macroquad::prelude::*;

pub async fn init_board(tiles: &mut Vec<Tile>){
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