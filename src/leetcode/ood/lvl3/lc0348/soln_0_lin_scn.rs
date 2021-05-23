/// https://leetcode.com/problems/design-tic-tac-toe/
///
/// Time Complexity:    O(1)
/// Space Complexity:   O(`n`)
///
/// Reference:
/// https://leetcode.com/problems/design-tic-tac-toe/discuss/81898/Java-O(1)-solution-easy-to-understand
#[allow(dead_code)]
struct TicTacToe {
    rows: Vec<i32>,
    cols: Vec<i32>,
    diagonal: i32,
    anti_diagonal: i32,
    n: i32,
}

#[allow(dead_code)]
impl TicTacToe {
    /** Initialize your data structure here. */
    fn new(n: i32) -> Self {
        TicTacToe {
            rows: vec![0; n as usize],
            cols: vec![0; n as usize],
            diagonal: 0,
            anti_diagonal: 0,
            n: n,
        }
    }
    /** Player {player} makes a move at ({row}, {col}).
    @param row The row of the board.
    @param col The column of the board.
    @param player The player, can be either 1 or 2.
    @return The current winning condition, can be either:
            0: No one wins.
            1: Player 1 wins.
            2: Player 2 wins. */
    fn make_a_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        let row: usize = row as usize;
        let col: usize = col as usize;

        let to_add: i32 = if player == 1 { 1 } else { -1 };
        let target: i32 = if player == 1 { self.n } else { -self.n };
        self.rows[row] += to_add;
        if self.rows[row] == target {
            return player;
        }
        self.cols[col] += to_add;
        if self.cols[col] == target {
            return player;
        }

        if row == col {
            self.diagonal += to_add;
        }
        if self.diagonal == target {
            return player;
        }

        if (row + col) as i32 == self.n - 1 {
            self.anti_diagonal += to_add;
        }
        if self.anti_diagonal == target {
            return player;
        }
        0
    }
}
