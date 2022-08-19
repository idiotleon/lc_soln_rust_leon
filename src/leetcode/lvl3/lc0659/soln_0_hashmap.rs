use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/split-array-into-consecutive-subsequences/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/split-array-into-consecutive-subsequences/discuss/106493/c-on-solution-two-pass
/// https://leetcode.com/problems/split-array-into-consecutive-subsequences/discuss/106514/C++Python-Esay-Understand-Solution/503317
/// https://leetcode.com/problems/split-array-into-consecutive-subsequences/discuss/106514/C%2B%2BPython-Esay-Understand-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let len_ns: usize = nums.len();
        let mut cnt: HashMap<i32, i32> = HashMap::with_capacity(len_ns);
        let mut tails: HashMap<i32, i32> = HashMap::with_capacity(len_ns);
        for &num in &nums {
            *cnt.entry(num).or_default() += 1;
        }
        for &num in &nums {
            if let Some(&f) = cnt.get(&num) {
                if f <= 0 {
                    continue;
                }
            }
            *cnt.entry(num).or_default() -= 1;
            if *tails.get(&(num - 1)).unwrap_or(&0) > 0 {
                *tails.entry(num - 1).or_default() -= 1;
                *tails.entry(num).or_default() += 1;
            } else if *cnt.get(&(num + 1)).unwrap_or(&0) > 0
                && *cnt.get(&(num + 2)).unwrap_or(&0) > 0
            {
                *cnt.entry(num + 1).or_default() -= 1;
                *cnt.entry(num + 2).or_default() -= 1;
                *tails.entry(num + 2).or_default() += 1;
            } else {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_test_case_73_should_return_expected() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 4, 5];
        let expected: bool = false;
        let actual: bool = Solution::is_possible(nums);
        assert_eq!(expected, actual);
    }
}
