/// @author: Leon
/// https://leetcode.com/problems/cherry-pickup-ii/
/// Time Complexity:    O(9 * `len_r` * (`len_c` ^ 2)) ~ O(`len_r` * (`len_c` ^ 2))
/// Space Complexity:   O(`len_r` * (`len_c` ^ 2))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        let mut memo: Vec<Vec<Vec<Option<i32>>>> = vec![vec![vec![None; len_c]; len_c]; len_r];
        Self::dfs(0, 0, len_c - 1, &grid, &mut memo)
    }
    fn dfs(
        r: usize,
        c1: usize,
        c2: usize,
        grid: &Vec<Vec<i32>>,
        memo: &mut Vec<Vec<Vec<Option<i32>>>>,
    ) -> i32 {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        if r == len_r {
            return 0;
        }
        if let Some(m) = memo[r][c1][c2] {
            return m;
        }
        let mut cherries: i32 = 0;
        for delta1 in -1..=1 as isize {
            for delta2 in -1..=1 as isize {
                let c1_nxt = c1 as isize + delta1;
                let c2_nxt = c2 as isize + delta2;
                if c1_nxt < 0 || c2_nxt < 0 || c1_nxt as usize >= len_c || c2_nxt as usize >= len_c
                {
                    continue;
                }
                cherries = std::cmp::max(
                    cherries,
                    Self::dfs(r + 1, c1_nxt as usize, c2_nxt as usize, grid, memo),
                );
            }
        }
        cherries += if c1 == c2 {
            grid[r][c1]
        } else {
            grid[r][c1] + grid[r][c2]
        };
        memo[r][c1][c2] = Some(cherries);
        cherries
    }
}
