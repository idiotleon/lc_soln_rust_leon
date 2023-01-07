/// @author: Leon
/// https://leetcode.com/problems/valid-perfect-square/
/// Time Complexity:    O(sqrt(`num`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/valid-perfect-square/solutions/83874/a-square-number-is-1-3-5-7-java-code/?orderBy=hot
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_perfect_square(mut num: i32) -> bool {
        let mut n: i32 = 1;
        while num > 0 {
            num -= n;
            n += 2;
        }
        return num == 0;
    }
}
