use std::collections::HashMap;

/// https://leetcode.com/problems/intervals-between-identical-elements/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
/// Reference:
/// https://leetcode.com/problems/intervals-between-identical-elements/discuss/1647522/Java-or-TC%3A-O(N)-or-Simple-or-Explanation
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_distances(nums: Vec<i32>) -> Vec<i64> {
        let len_n: usize = nums.len();
        let mut ans: Vec<i64> = vec![0; len_n];
        let mut num_to_indices_sum: HashMap<i32, i64> = HashMap::new();
        let mut num_to_freq: HashMap<i32, i32> = HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            ans[idx] += (idx as i64) * (*num_to_freq.entry(num).or_insert(0) as i64)
                - *num_to_indices_sum.entry(num).or_insert(0);
            *num_to_indices_sum.entry(num).or_default() += idx as i64;
            *num_to_freq.entry(num).or_default() += 1;
        }
        num_to_indices_sum.clear();
        num_to_freq.clear();
        for (idx, &num) in nums.iter().rev().enumerate() {
            // to switch to left/start-right/end indexing,
            // otherwise, it is by default, because of `.rev()`, from right/end to left/start
            let idx = len_n - idx - 1;
            ans[idx] += (len_n - idx - 1) as i64 * (*num_to_freq.entry(num).or_insert(0) as i64)
                - *num_to_indices_sum.entry(num).or_insert(0);
            *num_to_indices_sum.entry(num).or_default() += (len_n - idx - 1) as i64;
            *num_to_freq.entry(num).or_default() += 1;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1() {
        let nums: Vec<i32> = vec![2, 1, 3, 1, 2, 3, 3];
        let actual: Vec<i64> = Solution::get_distances(nums);
        let expected: Vec<i64> = vec![4, 2, 7, 2, 4, 4, 5];
        assert_eq!(expected, actual);
    }
}
