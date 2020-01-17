use std::io;

#[derive(Clone, Debug)]
enum Player {
  XPlayer,
  CirclePlayer,
  Empty
}

#[derive(Debug)]
struct Game {
  board: Board,
  current_player: Player
}

impl Game {
  fn new() -> Game {
    Game {
      board: Board::new(),
      current_player: Player::XPlayer
    }
  }

  fn toggle_player(&mut self) {
    if let Player::XPlayer = self.current_player {
      self.current_player = Player::CirclePlayer.clone()
    } else {
      self.current_player = Player::XPlayer.clone()
    }
  }

  fn do_move(&mut self, pos: usize) -> bool {
    if self.board.free_position(pos) {
      self.board.set_player_at_position(&self.current_player, pos);
      self.toggle_player();
      true
    } else {
      false
    }
  }

  fn show_board(&self) {
    println!("\n-------|-------|-------");

    for row in 0..3 {
      println!("       |       |       ");

      for col in 0..3 {
        let position_number = Board::calculate_position(row, col) + 1;

        if col == 1 {
          match self.board.get_player_at_row_col(row, col) {
            Player::Empty => print!("|   {}   |", position_number),
            Player::XPlayer => print!("|   X   |"),
            Player::CirclePlayer => print!("|   O   |")
          }
        } else {
          match self.board.get_player_at_row_col(row, col) {
            Player::Empty => print!("   {}   ", position_number),
            Player::XPlayer => print!("   X   "),
            Player::CirclePlayer => print!("   O   ")
          }
        }
      }

      println!("\n       |       |       ");
      println!("-------|-------|-------");
    }
  }

  fn show_free_positions(&self) {
    let free = self.board.free_positions();

    print!("The available positions are: ");

    for pos in 0..free.len() {
      if let Some(free_pos) = free.get(pos) {
        print!("{}", free_pos + 1);

        if pos == (free.len() - 1) {
          print!("\n");
        } else {
          print!(", ");
        }
      }
    }
  }

  fn start() {
    let mut game = Game::new();
    let mut counter = 0;

    'outer: loop {
      if counter == 9 {
        game.show_board();
        println!("Nobody won");
        break;
      }

      let current_player = match game.current_player {
        Player::XPlayer => "X",
        Player::CirclePlayer => "O",
        Player::Empty => unreachable!()
      };

      'inner: loop {
        game.show_board();
        game.show_free_positions();

        println!("Player {}, which position do you want to pick? ", current_player);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: i32 = input.trim().parse().unwrap();

        if input >= 0 && input <= 8 {
          if game.do_move((input - 1) as usize) {
            break 'inner;
          } else {
            println!("Position {} is already occupied", input);
          }
        } else {
          println!("{} is not a valid position", input);
        }
      }

      match game.board.check_winner() {
        Player::XPlayer => {
          game.show_board();
          println!("X Wins!");
          break 'outer;
        },
        Player::CirclePlayer => {
          game.show_board();
          println!("Circle Wins!");
          break 'outer;
        },
        Player::Empty => {
          counter += 1;
          continue;
        }
      }
    }
  }
}

#[derive(Debug)]
struct Board {
  positions: [Player; 9]
}

impl Board {
  fn new() -> Board {
    Board {
      positions: [Player::Empty, Player::Empty, Player::Empty,
                  Player::Empty, Player::Empty, Player::Empty,
                  Player::Empty, Player::Empty, Player::Empty]
    }
  }

  fn free_position(&self, pos: usize) -> bool {
    if let Player::Empty = self.get_player_at_position(pos) {
      true
    } else {
      false
    }
  }

  fn calculate_position(row: usize, col: usize) -> usize {
    (3 * row) + col
  }

  fn get_player_at_position(&self, pos: usize) -> &Player {
    &self.positions[pos]
  }

  fn get_player_at_row_col(&self, row: usize, col: usize) -> &Player {
    let pos = Board::calculate_position(row, col);

    self.get_player_at_position(pos)
  }

  fn set_player_at_position(&mut self,  player: &Player, pos: usize) {
    self.positions[pos] = player.clone();
  }

  fn free_positions(&self) -> Vec<usize> {
    let mut free = Vec::new();

    for index in 0..self.positions.len() {
      if self.free_position(index) {
        free.push(index)
      }
    }

    free
  }

  fn check_winner(&self) -> Player {
    let three_in_a_rows = [[0, 1, 2], [0, 3, 6], [0, 4, 8], [1, 4, 7],
                           [2, 5, 8], [2, 4, 6], [3, 4, 5], [6, 7, 8]];

    for triple_index in 0..three_in_a_rows.len() {
      let triple = three_in_a_rows[triple_index];

      let first = triple[0];
      let second = triple[1];
      let third = triple[2];
      let first = &self.positions[first];
      let second = &self.positions[second];
      let third = &self.positions[third];

      match (first, second, third) {
        (Player::XPlayer, Player::XPlayer, Player::XPlayer) => return Player::XPlayer,
        (Player::CirclePlayer, Player::CirclePlayer, Player::CirclePlayer) => return Player::CirclePlayer,
        _ => Player::Empty
      };   
    }

    Player::Empty
  }
}

fn main() {
  Game::start();
}
