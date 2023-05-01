/// @author: Leon
/// https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/
/// Time Complexity:    O(`len_ss`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn average(salaries: Vec<i32>) -> f64 {
        let len_ss: usize = salaries.len();
        const RANGE: i32 = 1e6 as i32 + 7;
        let mut max: i32 = -1;
        let mut min: i32 = RANGE;
        let mut sum: f64 = 0.0;
        for salary in salaries {
            max = std::cmp::max(max, salary);
            min = std::cmp::min(min, salary);
            sum += salary as f64;
        }
        return (sum - max as f64 - min as f64) / (len_ss - 2) as f64;
    }
}
