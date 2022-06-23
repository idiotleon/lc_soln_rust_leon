use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/append-k-integers-with-minimal-sum/
/// Time Complexity:    O(`_len_n` * lg(`_len_n`))
/// Space Complexity:   O(`_len_n`)
/// Reference:
/// https://leetcode.com/problems/append-k-integers-with-minimal-sum/discuss/1823848/Java-solution-use-n*(n%2B1)2
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let _len_n: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut sorted = nums;
            sorted.sort();
            sorted
        };
        let mut sum: i64 = 0;
        let mut k: i64 = k as i64;
        let mut seen: HashSet<i64> = HashSet::new();
        for num in sorted {
            let num: i64 = num as i64;
            if seen.insert(num) && num <= k {
                k += 1;
                sum += num;
            }
        }
        (1 + k) * k / 2 - sum
    }
}
