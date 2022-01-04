/// https://leetcode.com/problems/complement-of-base-10-integer/
/// Time Complexity:    O(lg(`n`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/complement-of-base-10-integer/discuss/256740/JavaC%2B%2BPython-Find-111.....1111-greater-N
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let mut bit: i32 = 1;
        while n > bit {
            bit = bit * 2 + 1;
        }
        n ^ bit
    }
}
