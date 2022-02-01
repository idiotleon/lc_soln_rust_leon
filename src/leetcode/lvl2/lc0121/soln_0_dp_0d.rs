/// @author: Leon
/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
/// Time Complexity:    O(`_len_ps`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/discuss/39038/Kadane's-Algorithm-Since-no-one-has-mentioned-about-this-so-far-%3A)-(In-case-if-interviewer-twists-the-input)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let _len_ps: usize = prices.len();
        const RANGE: u16 = 1e4 as u16 + 7;
        let mut min_price: u16 = RANGE;
        let mut max_profit: u16 = 0;
        for price in prices {
            let price: u16 = price as u16;
            if price < min_price {
                min_price = price;
            }
            let profit = price - min_price;
            if profit > max_profit {
                max_profit = profit;
            }
        }
        max_profit as i32
    }
}
