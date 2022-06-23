use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/discuss/91049/Java-O(n)-solution-using-bit-manipulation-and-HashMap/95535
/// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/discuss/91049/Java-O(n)-solution-using-bit-manipulation-and-HashMap
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut max: i32 = 0;
        let mut mask: i32 = 0;
        for idx in (0..32).rev() {
            mask = mask | (1 << idx);
            let mut seen: HashSet<i32> = HashSet::new();
            for &num in &nums {
                seen.insert(num & mask);
            }
            let tmp = max | (1 << idx);
            for &prefix in &seen {
                if seen.contains(&(tmp ^ prefix)) {
                    max = tmp;
                    break;
                }
            }
        }
        max
    }
}
