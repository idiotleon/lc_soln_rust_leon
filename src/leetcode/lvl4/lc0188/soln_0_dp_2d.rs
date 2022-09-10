/// @author: Leon
/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/
/// Time Complexity:    O(`k` * `len_ds`)
/// Space Complexity:   O(`k` * `len_ds`)
/// Reference:
/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/discuss/54117/Clean-Java-DP-solution-with-comment
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let len_ds: usize = prices.len();
        let k: usize = k as usize;
        if k >= len_ds {
            let max_profit: i32 = {
                let mut profit: i32 = 0;
                for day in 1..len_ds {
                    if prices[day - 1] < prices[day] {
                        profit += prices[day] - prices[day - 1];
                    }
                }
                profit
            };
            return max_profit;
        }
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len_ds]; k + 1];
        for ki in 1..=k {
            let mut local_max: i32 = dp[k - 1][0] - prices[0];
            for day in 1..len_ds {
                dp[ki][day] = std::cmp::max(dp[ki][day - 1], prices[day] + local_max);
                local_max = std::cmp::max(local_max, dp[ki - 1][day] - prices[day]);
            }
        }
        return dp[k][len_ds - 1];
    }
}
