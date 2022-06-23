use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/powerful-integers/
/// Time Complexity:     O()
/// Space Complexity:    O()
/// Reference:
/// https://leetcode.com/problems/powerful-integers/discuss/1183644/C%2B%2B-ans-Rust-bruteforce
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut set: HashSet<i32> = HashSet::new();
        let mut lo = 1;
        while lo + 1 <= bound {
            let mut hi = 1;
            while lo + hi <= bound {
                set.insert(lo + hi);
                hi *= y;
                if y == 1 {
                    break;
                }
            }
            lo *= x;
            if x == 1 {
                break;
            }
        }
        set.into_iter().collect()
    }
}
