use macroquad::prelude::*;
use macroquad::color::Color;

pub struct Tile {
  pub rect: Rect,
  pub color: Color
}

impl Tile {
  pub fn new(pos: Vec2, size: Vec2, color: Color) -> Self {
    Self{ rect: Rect::new(pos.x, pos.y, size.x, size.y), color}
  }

  pub fn draw(&self) {
    draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLACK);
  }
}