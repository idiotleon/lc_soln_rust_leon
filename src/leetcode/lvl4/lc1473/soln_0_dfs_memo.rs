/// @author: Leon
/// https://leetcode.com/problems/paint-house-iii/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/paint-house-iii/discuss/674413/C++-Top-Down-DP/569880
/// https://leetcode.com/problems/paint-house-iii/discuss/674413/C%2B%2B-Top-Down-DP
struct Solution;

#[allow(dead_code)]
impl Solution {
    const RANGE: i32 = 1e6 as i32 + 7;
    pub fn min_cost(houses: Vec<i32>, costs: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let mut memo: Vec<Vec<Vec<Option<i32>>>> =
            vec![vec![vec![None; n as usize + 1]; target as usize + 1]; m as usize + 1];
        let min_cost = Self::dfs(0, 0, target, &houses, &costs, &mut memo);
        if min_cost == Self::RANGE {
            -1
        } else {
            min_cost
        }
    }
    fn dfs(
        idx_house: usize,
        idx_prev_color: usize,
        target: i32,
        houses: &Vec<i32>,
        costs: &Vec<Vec<i32>>,
        memo: &mut Vec<Vec<Vec<Option<i32>>>>,
    ) -> i32 {
        let len_hs: usize = houses.len();
        if idx_house >= len_hs || target < 0 {
            return if target == 0 { target } else { Self::RANGE };
        }
        let len_cls: usize = costs[idx_house].len();
        if let Some(m) = memo[idx_house][target as usize][idx_prev_color] {
            return m;
        }
        if houses[idx_house] != 0 {
            return Self::dfs(
                1 + idx_house,
                houses[idx_house] as usize,
                target
                    - if idx_prev_color == houses[idx_house] as usize {
                        0
                    } else {
                        1
                    },
                houses,
                costs,
                memo,
            );
        }
        let mut min_cost: i32 = Self::RANGE;
        for idx_cur_color in 1..=len_cls {
            let cur_cost = costs[idx_house][idx_cur_color - 1]
                + Self::dfs(
                    idx_house + 1,
                    idx_cur_color,
                    target
                        - if idx_prev_color == idx_cur_color {
                            0
                        } else {
                            1
                        },
                    houses,
                    costs,
                    memo,
                );
            min_cost = std::cmp::min(min_cost, cur_cost);
        }
        memo[idx_house][target as usize][idx_prev_color] = Some(min_cost);
        min_cost
    }
}
