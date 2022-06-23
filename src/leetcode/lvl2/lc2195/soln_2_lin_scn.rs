use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/append-k-integers-with-minimal-sum/
/// Time Complexity:    O(`RANGE`)
/// Space Complexiyt:   O(1)
/// This approach gets TLEed
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        const RANGE: i32 = 1e9 as i32 + 7;
        let mut sum = 0;
        // let mut sum: i64 = nums.iter().map(|&e| e as i64).sum();
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut cnt: i32 = 1;
        for num in 1..RANGE {
            if set.contains(&num) {
                continue;
            }
            sum += num as i64;
            cnt += 1;
            if cnt > k {
                break;
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums: Vec<i32> = vec![1, 4, 25, 10, 25];
        let k: i32 = 2;
        let actual: i64 = Solution::minimal_k_sum(nums, k);
        let expected: i64 = 5;
        assert_eq!(expected, actual);
    }
}
