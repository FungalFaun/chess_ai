pub const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn parse_fen(fen_str: &str) -> Vec<&str> {
  let state: Vec<&str> = fen_str.split(' ').collect();
  let positions: Vec<&str> = state[0].split('/').collect();

  let mut chars = Vec::new();

  for pos in positions {
    match pos.parse::<u32>() {
      Ok(num) => replace_with_space(num, &mut chars),
      Err(_) => chars.push(pos)
    }
  }

  chars
}

fn replace_with_space(num: u32, chars: &mut Vec<&str>) {
  for _i in 0..num {
    chars.push("_");
  }
}