/// @author: Leon
/// https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/
/// Time Complexity:    O(`len_p`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/discuss/1635083/PythonC++Java-One-Pass-DP-oror-O(N)-oror-Detailed-Explanation-oror-Clean-and-Easy-to-Understand/1184977
/// https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/discuss/1635033/C%2B%2B-O(N)-time-one-pass
/// https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/discuss/1635083/PythonC%2B%2BJava-One-Pass-DP-oror-O(N)-oror-Detailed-Explanation-oror-Clean-and-Easy-to-Understand
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let len_p = prices.len();
        let mut ans: i64 = 1;
        let mut len: i64 = 1;
        for idx in 1..len_p{
            if prices[idx - 1] - prices[idx] == 1{
                len += 1;
            }else{
                len = 1;
            }
            ans += len;
        }
        ans
    }
}