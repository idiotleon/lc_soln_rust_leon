use std::collections::HashMap;
/// https://leetcode.com/problems/longest-consecutive-sequence/
/// Time Complexity:    O()
/// Space Complexity:   O()
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut num_to_seq_len: HashMap<i32, u32> = HashMap::new();
        let mut longest: u32 = 0;
        for num in nums {
            if num_to_seq_len.contains_key(&num) {
                continue;
            }
            let lo = *num_to_seq_len.get(&(num - 1)).unwrap_or(&0);
            let hi = *num_to_seq_len.get(&(num + 1)).unwrap_or(&0);
            let len = lo + hi + 1;
            num_to_seq_len.insert(num, len);
            longest = std::cmp::max(longest, len);
            if lo > 0 {
                num_to_seq_len.insert(num - lo as i32, len);
            }
            if hi > 0 {
                num_to_seq_len.insert(num + hi as i32, len);
            }
        }
        longest as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1() {
        let nums: Vec<i32> = vec![100, 4, 200, 1, 3, 2];
        let actual: i32 = Solution::longest_consecutive(nums);
        let expected: i32 = 4;
        assert_eq!(expected, actual);
    }
}
