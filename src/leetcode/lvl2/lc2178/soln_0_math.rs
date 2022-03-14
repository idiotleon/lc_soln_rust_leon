/// @author: Leon
/// https://leetcode.com/problems/maximum-split-of-positive-even-integers/
/// Time Complexity:    O(`final_sum` ^ 0.5)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/maximum-split-of-positive-even-integers/discuss/1783317/JavaPython-3-Greedy-w-brief-explanation-and-analysis.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_even_split(mut final_sum: i64) -> Vec<i64> {
        let mut ans: Vec<i64> = Vec::new();
        if final_sum % 2 == 0 {
            let mut factor: i64 = 2;
            while factor <= final_sum {
                ans.push(factor);
                final_sum -= factor;
                factor += 2;
            }
            if let Some(last) = ans.pop() {
                ans.push(last + final_sum);
            }
        }
        ans
    }
}
