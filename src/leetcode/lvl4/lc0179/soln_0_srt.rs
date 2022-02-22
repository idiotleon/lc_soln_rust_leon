/// @author: Leon
/// https://leetcode.com/problems/largest-number/
/// Time Complexity:    O(`_len_n` * lg(`_len_n`) * avg_len)
/// Space Complexity:   O(`_len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let _len_n: usize = nums.len();
        let sorted: Vec<String> = {
            let mut strs: Vec<String> = nums.into_iter().map(|n| n.to_string()).collect();
            strs.sort_by(|a, b| (b.to_owned() + a).cmp(&(a.to_owned() + b)));
            strs
        };
        if '0' == sorted[0].chars().nth(0).unwrap() {
            "0".to_owned()
        } else {
            sorted.join("")
        }
    }
}
