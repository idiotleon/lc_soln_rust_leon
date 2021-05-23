/// https://leetcode.com/problems/n-queens/
///
/// Time Complexity:    O(`n`!)
/// Space Complexity:   O(`n`)
///
/// Reference:
/// https://leetcode.com/problems/n-queens/discuss/19808/Accepted-4ms-c%2B%2B-solution-use-backtracking-and-bitmask-easy-understand.
/// https://mp.weixin.qq.com/s?__biz=MzAxODQxMDM0Mw==&mid=2247484709&idx=1&sn=1c24a5c41a5a255000532e83f38f2ce4&chksm=9bd7fb2daca0723be888b30345e2c5e64649fc31a00b05c27a0843f349e2dd9363338d0dac61&scene=178&cur_album_id=1318883740306948097#rd
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    const QUEEN: char = 'Q';
    const EMPTY: char = '.';
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n: usize = n as usize;
        let mut is_same_col: Vec<bool> = vec![false; n];
        let mut is_same_main_diagonal: Vec<bool> = vec![false; 2 * n - 1];
        let mut is_same_anti_diagonal: Vec<bool> = vec![false; 2 * n - 1];
        let mut board: Vec<Vec<char>> = vec![vec!['.'; n]; n];
        let mut ans: Vec<Vec<String>> = Vec::new();

        Self::backtrack(
            0,
            &mut is_same_col,
            &mut is_same_main_diagonal,
            &mut is_same_anti_diagonal,
            &mut board,
            n,
            &mut ans,
        );
        ans
    }
    fn backtrack(
        row: usize,
        is_same_col: &mut Vec<bool>,
        is_same_main_diagonal: &mut Vec<bool>,
        is_same_anti_diagonal: &mut Vec<bool>,
        board: &mut Vec<Vec<char>>,
        n: usize,
        res: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            res.push(
                board
                    .iter()
                    .map(|cur_row| cur_row.iter().collect())
                    .collect(),
            );
            return;
        }

        for col in 0..n {
            if is_same_col[col]
                || is_same_main_diagonal[row + n - col - 1]
                || is_same_anti_diagonal[row + col]
            {
                continue;
            }

            is_same_col[col] = true;
            is_same_main_diagonal[row + n - col - 1] = true;
            is_same_anti_diagonal[row + col] = true;
            board[row][col] = Self::QUEEN;

            Self::backtrack(
                row + 1,
                is_same_col,
                is_same_main_diagonal,
                is_same_anti_diagonal,
                board,
                n,
                res,
            );

            is_same_col[col] = false;
            is_same_main_diagonal[row + n - col - 1] = false;
            is_same_anti_diagonal[row + col] = false;
            board[row][col] = Self::EMPTY;
        }
    }
}
