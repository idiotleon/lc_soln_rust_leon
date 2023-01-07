/// @author: Leon
/// https://leetcode.com/problems/valid-perfect-square/
/// Time Complexity:    O()
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/valid-perfect-square/solutions/83874/a-square-number-is-1-3-5-7-java-code/?orderBy=hot
/// https://en.wikipedia.org/wiki/Integer_square_root#Using_only_integer_division
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num: i64 = num as i64;
        let mut n: i64 = num;
        while n * n > num {
            n = (n + num / n) >> 1;
        }
        return n * n == num;
    }
}
