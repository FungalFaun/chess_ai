pub const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn parse_fen(fen_str: &str){
  let state: Vec<&str> = fen_str.split(' ').collect();
  let positions: Vec<&str> = state[0].split('/').collect();

  let mut parsed_position: Vec<&str> = Vec::new();

  let mut numbers: Vec<i32> = Vec::new();
  let mut chars = Vec::new();

  for pos in positions {
    match pos.parse() {
      Ok(num) => numbers.push(num),
      Err(_) => chars.push(pos)
    }
  }

  println!("Nums: {:#?}", numbers);
  println!("Chars: {:#?}", chars);
}
