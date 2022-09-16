use macroquad::texture::Image;
use macroquad::prelude::*;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum PieceType {
  Pawn = 5,
  Knight = 3,
  Bishop = 2,
  Rook = 4,
  Queen = 1,
  King = 0
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum PieceColor {
  White = 0,
  Black = 1
}

pub struct Piece {
  pub image: Image,
  pub size: f32,
  pub pos: Vec2,
  pub piece: PieceType,
  pub color: PieceColor
}

impl Piece {
  pub fn new(pos: Vec2, piece_type: PieceType, color: PieceColor, size: f32) -> Self {

    let image = Piece::get_image(
      piece_type as isize as f32, 
      color as isize as f32);
  
    Self { 
      image, 
      size,
      pos: vec2(pos.x, pos.y), 
      piece: piece_type, color }
  }

  fn get_image(x: f32, y: f32) -> Image {
    let base = Image::from_file_with_format(
      include_bytes!("../../assets/pieces.png"),
      Some(ImageFormat::Png)
    );

    let piece_size = vec2(
      (base.width / 6u16) as f32, 
      (base.height / 2u16) as f32);

    base.sub_image(
      Rect::new(
        x * piece_size.x, 
        y * piece_size.y, 
        piece_size.x,
        piece_size.y
        )
      )
  }

  pub fn draw(&self) {
    draw_texture_ex(Texture2D::from_image(&self.image), self.pos.x, self.pos.y, WHITE, 
    DrawTextureParams {
      dest_size: Some(vec2(self.size, self.size)),
      source: None,
      flip_x: false,
      flip_y: false,
      pivot: None,
      rotation: 0f32
  });
  }
}