/// https://leetcode.com/problems/number-complement/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
/// Reference:
///  https://leetcode.com/problems/number-complement/discuss/95992/Java-1-line-bit-manipulation-solution/100579
/// https://leetcode.com/problems/number-complement/discuss/95992/Java-1-line-bit-manipulation-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mask: i32 = {
            let mut mask: i32 = 0;
            while mask < num {
                mask = (mask << 1) | 1;
            }
            mask
        };
        !num & mask
    }
}
