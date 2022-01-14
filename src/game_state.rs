use wasm_bindgen::prelude::*;

// enum CellState {
//     X,
//     O
//     // Empty
// }

// enum Index {
//     1,
//     2,
//     3
// }

#[derive(PartialEq, Copy, Clone)]
enum Player {
    X,
    O
}

type Board = [[Option<Player>; 3]; 3];

#[wasm_bindgen] 
pub struct GameStatusUpdate(pub bool, pub char, pub bool);

#[wasm_bindgen]
pub struct GameState { 
  current_player: Player,
  game_over: bool,
    board: Board,  
}

#[wasm_bindgen]
impl GameState {
    pub fn new() -> GameState { 

        GameState {
            current_player: Player::X,
            game_over: false,
            board: [ 
                [ None, None, None],
                [ None, None, None],
                [ None, None, None], 
            ], 
        }
    }

    pub fn player_move(&mut self, row: usize, col: usize) -> GameStatusUpdate { 
        let player_curr = GameState::player_to_char(self.current_player);
        let val = &self.board[row][col];
        if val.is_none() && !self.game_over {
            self.board[row][col] = Some(self.current_player);
            // toggle active player 
            self.current_player = if self.current_player == Player::X { Player::O} else { Player::X};
            // self.current_player = Player::O;
            let player_char = GameState::player_to_char(self.current_player);
            let is_over = Self::is_game_over(&self.board);
            return GameStatusUpdate(true, player_char, is_over);  
        } 
        return GameStatusUpdate(false, player_curr, false); 
 
    }

    fn player_to_char(p: Player) -> char {
        let c: char = if p == Player::X { 'O' } else { 'X' };
        return c;
    }

    fn is_game_over(b: &Board) -> bool { 
        let grid = b;
        
        let mut i = 0;
        while i < 3 {
            // check rows
            let row = grid[i];
            if row[0].is_some() && row[0] == row[1] && row[1] == row[2]{
                return true;
            }
            // check cols
            let mut j = 0;
            if grid[0][i].is_some() && grid[0][i] == grid[1][i] && grid[1][i] == grid[2][i] {
                return true;
            }
            i += 1;
        } 

        // diagonals
        if grid[0][0].is_some() && grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
            return true;
        }
        if grid[0][2].is_some() && grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] {
            return true;
        }
        return false;
    }
 
}