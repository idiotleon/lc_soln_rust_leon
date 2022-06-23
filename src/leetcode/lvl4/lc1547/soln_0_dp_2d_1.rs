/// @author: Leon
/// https://leetcode.com/problems/minimum-cost-to-cut-a-stick/
/// Time Complexity:    O(`len_p` ^ 3)
/// Space Complexity:   O(`len_p` ^ 2)
///
/// this is NOT yet correct solution
///
/// Reference:
/// https://leetcode.com/problems/minimum-cost-to-cut-a-stick/discuss/780880/DP-with-picture-(Burst-Balloons)
/// https://leetcode.com/problems/minimum-cost-to-cut-a-stick/discuss/781074/JavaC%2B%2BPython-Merge-Stones
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let len_c: usize = cuts.len();
        let len_p: usize = len_c + 2;
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
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len_p]; len_p];
        for lo in (0..len_p).rev() {
            for hi in lo + 1..len_p {
                // why does it lead to integer overflow?
                // let mut min_cost = std::i32::MAX;
                let mut min_cost: i32 = 1e9 as i32 + 7;
                for mid in lo + 1..hi {
                    min_cost = std::cmp::min(
                        min_cost,
                        padded[hi] - padded[lo] + dp[lo][mid] + dp[mid][hi],
                    );
                }
                dp[lo][hi] = min_cost;
            }
        }
        dp[0][len_p - 1]
    }
}
