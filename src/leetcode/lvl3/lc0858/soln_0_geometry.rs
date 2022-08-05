/// @author: Leon
/// https://leetcode.com/problems/mirror-reflection/
/// Time Complexity:    O(lg(max(`p`, `q`)))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/mirror-reflection/discuss/2376191/C%2B%2B-Java-Python-or-Faster-then-100-or-Full-explanations-or
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn mirror_reflection(mut p: i32, mut q: i32) -> i32 {
        while p % 2 == 0 && q % 2 == 0 {
            p /= 2;
            q /= 2;
        }
        return 1 - p % 2 + q % 2;
    }
}
