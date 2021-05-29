/// https://leetcode.com/problems/n-queens-ii/
///
/// Time Complexity:    O(`n`!)
/// Space Complexity:   O(`n`)
///
/// Reference:
/// https://leetcode.com/problems/n-queens-ii/discuss/298639/Rust-DFS-and-BitWis-0ms-Solution
use std::collections::HashSet;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    const QUEEN: char = 'Q';
    const EMPTY: char = '.';
    pub fn total_n_queens(n: i32) -> i32 {
        let n: usize = n as usize;

        let mut cols = HashSet::<usize>::new();
        let mut dia_sum = HashSet::<usize>::new();
        let mut anti_dia_sum = HashSet::<usize>::new();
        let mut board: Vec<Vec<char>> = vec![vec!['.'; n]; n];

        Self::backtrack(0, &mut cols, &mut dia_sum, &mut anti_dia_sum, &mut board, n)
    }
    fn backtrack(
        row: usize,
        mut cols: &mut HashSet<usize>,
        mut dia_sum: &mut HashSet<usize>,
        mut anti_dia_sum: &mut HashSet<usize>,
        mut board: &mut Vec<Vec<char>>,
        n: usize,
    ) -> i32 {
        if row == n {
            return 1;
        }

        let mut cnt: i32 = 0;
        for col in 0..n {
            if cols.contains(&col)
                || dia_sum.contains(&(row + col))
                || anti_dia_sum.contains(&(row - col))
            {
                continue;
            }

            cols.insert(col);
            dia_sum.insert(row + col);
            anti_dia_sum.insert(row - col);
            board[row][col] = Self::QUEEN;

            cnt += Self::backtrack(
                row + 1,
                &mut cols,
                &mut dia_sum,
                &mut anti_dia_sum,
                &mut board,
                n,
            );

            cols.remove(&col);
            dia_sum.remove(&(row + col));
            anti_dia_sum.remove(&(row - col));
            board[row][col] = Self::EMPTY;
        }

        cnt
    }
}
