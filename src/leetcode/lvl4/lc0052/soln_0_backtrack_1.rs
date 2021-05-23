/// https://leetcode.com/problems/n-queens-ii/
///
/// Time Complexity:    O(`n`!)
/// Space Complexity:   O(`n1)
///
/// Reference:
/// https://leetcode.com/problems/n-queens-ii/discuss/20058/Accepted-Java-Solution/172490
/// https://mp.weixin.qq.com/s?__biz=MzAxODQxMDM0Mw==&mid=2247484709&idx=1&sn=1c24a5c41a5a255000532e83f38f2ce4&chksm=9bd7fb2daca0723be888b30345e2c5e64649fc31a00b05c27a0843f349e2dd9363338d0dac61&scene=178&cur_album_id=1318883740306948097#rd
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    const QUEEN: char = 'Q';
    const EMPTY: char = '.';
    pub fn total_n_queens(n: i32) -> i32 {
        let n: usize = n as usize;
        let mut is_same_column: Vec<bool> = vec![false; n];
        let mut is_same_main_diagonal: Vec<bool> = vec![false; 2 * n - 1];
        let mut is_same_anti_diagonal: Vec<bool> = vec![false; 2 * n - 1];
        let mut board: Vec<Vec<char>> = vec![vec!['.'; n]; n];

        let mut count: i32 = 0;

        Self::backtrack(
            0,
            &mut is_same_column,
            &mut is_same_main_diagonal,
            &mut is_same_anti_diagonal,
            &mut board,
            n,
            &mut count,
        );
        count
    }
    fn backtrack(
        row: usize,
        is_same_column: &mut Vec<bool>,
        is_same_main_diagonal: &mut Vec<bool>,
        is_same_anti_diagonal: &mut Vec<bool>,
        board: &mut Vec<Vec<char>>,
        n: usize,
        count: &mut i32,
    ) {
        if row == n {
            *count += 1;
            return;
        }

        for col in 0..n {
            if is_same_column[col]
                || is_same_main_diagonal[row + n - col - 1]
                || is_same_anti_diagonal[row + col]
            {
                continue;
            }

            is_same_column[col] = true;
            is_same_main_diagonal[row + n - col - 1] = true;
            is_same_anti_diagonal[row + col] = true;
            board[row][col] = Self::QUEEN;

            Self::backtrack(
                row + 1,
                is_same_column,
                is_same_main_diagonal,
                is_same_anti_diagonal,
                board,
                n,
                count,
            );

            is_same_column[col] = false;
            is_same_main_diagonal[row + n - col - 1] = false;
            is_same_anti_diagonal[row + col] = false;
            board[row][col] = Self::EMPTY;
        }
    }
}
