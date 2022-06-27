/// @author: Leon
/// https://leetcode.com/problems/minimum-path-cost-in-a-grid/
/// Time Complexity:    O(`len_ps` ^ 2)
/// Space Complexity:   O(`len_ps` ^ 2)
/// Reference:
/// https://leetcode.com/problems/minimum-path-cost-in-a-grid/discuss/2141155/DP
/// https://leetcode.com/problems/minimum-path-cost-in-a-grid/discuss/2141640/C%2B%2B-Dijkstra-Algorithm-or-Short-overkill-but-100-runtime
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_costs: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid.len();
        const RANGE: i32 = 50 * 100 + 7;
        let len_vts: usize = grid[0].len();
        let mut dp: Vec<Vec<i32>> = {
            let mut dp = vec![vec![RANGE; len_vts]; len_rs];
            dp[0] = grid[0].to_vec();
            dp
        };
        for idx_r in 1..len_rs {
            for idx_c in 0..len_vts {
                for idx_v in 0..len_vts {
                    let cur_costs = &move_costs[grid[idx_r - 1][idx_c] as usize];
                    let vertex: i32 = grid[idx_r][idx_v];
                    dp[idx_r][idx_v] = std::cmp::min(
                        dp[idx_r][idx_v],
                        vertex + dp[idx_r - 1][idx_c] + cur_costs[idx_v],
                    );
                }
            }
        }
        *dp[len_rs - 1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let grid: Vec<Vec<i32>> = vec![vec![5, 3], vec![4, 0], vec![2, 1]];
        let move_cost: Vec<Vec<i32>> = vec![
            vec![9, 8],
            vec![1, 5],
            vec![10, 12],
            vec![18, 6],
            vec![2, 4],
            vec![14, 3],
        ];
        let actual = Solution::min_path_cost(grid, move_cost);
        let expected = 17;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let grid: Vec<Vec<i32>> = vec![vec![5, 1, 2], vec![4, 0, 3]];
        let move_cost: Vec<Vec<i32>> = vec![
            vec![12, 10, 15],
            vec![20, 23, 8],
            vec![21, 7, 1],
            vec![8, 1, 13],
            vec![9, 10, 25],
            vec![5, 3, 2],
        ];
        let actual = Solution::min_path_cost(grid, move_cost);
        let expected = 6;
        assert_eq!(expected, actual);
    }
}
