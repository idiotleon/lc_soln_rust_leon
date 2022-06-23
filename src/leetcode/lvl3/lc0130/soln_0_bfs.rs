use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/surrounded-regions/
/// Time Complexity:    O(`n_rows` * `n_cols`)
/// Space Complexity:   O(`n_rows` * `n_cols`)
/// Reference:
/// https://leetcode.com/problems/surrounded-regions/discuss/41649/My-BFS-solution-(C%2B%2B-28ms)

struct Solution;

const DIRS: &'static [i8] = &[0, -1, 0, 1, 0];

const CELL_O: &'static char = &'O';
const CELL_T: &'static char = &'T';
const CELL_X: &'static char = &'X';

#[allow(dead_code)]
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let n_rows = board.len();
        let n_cols = board[0].len();
        let mut queue: VecDeque<(usize, usize)> = {
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            for row in 0..n_rows {
                if board[row][0] == *CELL_O {
                    board[row][0] = *CELL_T;
                    queue.push_back((row, 0));
                }
                if board[row][n_cols - 1] == *CELL_O {
                    board[row][n_cols - 1] = *CELL_T;
                    queue.push_back((row, n_cols - 1));
                }
            }
            for col in 1..n_cols - 1 {
                if board[0][col] == *CELL_O {
                    board[0][col] = *CELL_T;
                    queue.push_back((0, col));
                }
                if board[n_rows - 1][col] == *CELL_O {
                    board[n_rows - 1][col] = *CELL_T;
                    queue.push_back((n_rows - 1, col));
                }
            }
            queue
        };
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                if let Some((row, col)) = queue.pop_front() {
                    for d in 0..4 {
                        let row_nxt = row as i8 + DIRS[d];
                        let col_nxt = col as i8 + DIRS[d + 1];
                        if row_nxt < 0
                            || col_nxt < 0
                            || row_nxt as usize >= n_rows
                            || col_nxt as usize >= n_cols
                            || board[row_nxt as usize][col_nxt as usize] != *CELL_O
                        {
                            continue;
                        }
                        board[row_nxt as usize][col_nxt as usize] = *CELL_T;
                        queue.push_back((row_nxt as usize, col_nxt as usize));
                    }
                };
            }
        }
        for row in board.iter_mut() {
            for col in 0..row.len() {
                if row[col] == *CELL_O {
                    row[col] = *CELL_X;
                }
                if row[col] == *CELL_T {
                    row[col] = *CELL_O;
                }
            }
        }
    }
}
