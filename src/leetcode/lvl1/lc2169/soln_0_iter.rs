/// @author: Leon
/// https://leetcode.com/problems/count-operations-to-obtain-zero/
/// Time Complexity:    O(lg(min(`num1`, `num2`)))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/count-operations-to-obtain-zero/discuss/1766803/JavaPython-3-Use-divmod-operation-w-explanation-and-analysis.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
        let mut cnt: i32 = 0;
        while std::cmp::min(num1, num2) > 0 {
            if num1 < num2 {
                let tmp = num1;
                num1 = num2;
                num2 = tmp;
            }
            cnt += num1 / num2;
            num1 %= num2;
        }
        cnt
    }
}
