/// @author: Leon
/// https://leetcode.com/problems/broken-calculator/
/// Time Complexity:    O(lg(`target`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/broken-calculator/discuss/1874851/oror-C%2B%2B-oror-Easy-Approach-oror-PROPER-EXPLANATION
/// https://leetcode.com/problems/broken-calculator/discuss/1874813/JavaC%2B%2B-Recursive-and-Iterative-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn broken_calc(start_value: i32, mut target: i32) -> i32 {
        let mut steps: i32 = 0;
        while target > start_value {
            if target % 2 == 0 {
                target /= 2;
            } else {
                target += 1;
            }
            steps += 1;
        }
        steps += start_value - target;
        steps
    }
}
