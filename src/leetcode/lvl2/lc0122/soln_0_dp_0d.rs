/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
/// Time Complexity:    O(`len_p`)
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len_p = prices.len();
        let max_profit = {
            let mut max_profit = 0;
            for idx in 1..len_p{
                if prices[idx - 1] < prices[idx]{
                    max_profit += prices[idx] - prices[idx - 1];
                }
            }
            max_profit
        };
        max_profit
    }
}