/// https://leetcode.com/problems/minimum-cost-to-cut-a-stick/
///
/// Time Complexity:    O(`len_p` ^ 3)
/// Space Complexity:   O(`len_p` ^ 2)
///
/// Reference:
/// https://leetcode.com/problems/minimum-cost-to-cut-a-stick/discuss/781074/JavaC++Python-Merge-Stones/648523
/// https://leetcode.com/problems/minimum-cost-to-cut-a-stick/discuss/780880/DP-with-picture-(Burst-Balloons)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let len_c = cuts.len();
        let len_p = len_c + 2;
        let padded: Vec<i32> = {
            let mut tmp: Vec<i32> = vec![0; len_p];
            tmp[0] = 0;
            tmp[len_p - 1] = n;
            for (idx, &num) in cuts.iter().enumerate() {
                tmp[idx + 1] = num;
            }
            tmp.sort();
            tmp
        };
        let mut memo: Vec<Vec<i32>> = vec![vec![0; len_p]; len_p];
        Self::dfs(0, len_p - 1, &padded, &mut memo)
    }

    fn dfs(lo: usize, hi: usize, padded: &Vec<i32>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if hi - lo <= 1 {
            return 0;
        }

        if memo[lo][hi] > 0 {
            return memo[lo][hi];
        }
        let mut min_cost: i32 = 1e9 as i32 + 7;
        for mid in lo + 1..hi {
            min_cost = std::cmp::min(
                min_cost,
                Self::dfs(lo, mid, padded, memo) + Self::dfs(mid, hi, padded, memo),
            );
        }
        min_cost += padded[hi] - padded[lo];
        memo[lo][hi] = min_cost;
        min_cost
    }
}
