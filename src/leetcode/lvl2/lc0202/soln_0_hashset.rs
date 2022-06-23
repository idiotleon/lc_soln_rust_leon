use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/happy-number/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut n = n;
        loop {
            seen.insert(n);
            // to compose the new `n`
            let mut sum = 0;
            while n != 0 {
                let digit = n % 10;
                sum += digit * digit;
                n /= 10;
            }
            n = sum;
            if (n == 1) || seen.contains(&n) {
                break;
            }
        }
        n == 1
    }
}
