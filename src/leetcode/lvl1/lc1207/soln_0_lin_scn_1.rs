use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/unique-number-of-occurrences/
/// Time Complexity:    O(`len_ns`) ~ O(`RANGE`) ~ O(1)
/// Space Complexity:   O(`RANGE`) ~ O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn unique_occurrences(nums: Vec<i32>) -> bool {
        const RANGE: usize = 1000 + 1;
        const PADDING: i32 = RANGE as i32;
        let freqs: Vec<u16> = {
            let mut freqs: Vec<u16> = vec![0; 2 * RANGE];
            for num in nums {
                freqs[(num + PADDING) as usize] += 1;
            }
            freqs
        };
        let mut seen: HashSet<u16> = HashSet::with_capacity(RANGE * 2);
        for freq in freqs {
            if freq == 0 {
                continue;
            }
            if !seen.insert(freq) {
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
    fn it_works_with_sample_input_1() {
        let nums: Vec<i32> = vec![1, 2, 2, 1, 1, 3];
        let expected: bool = true;
        let actual = Solution::unique_occurrences(nums);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let nums: Vec<i32> = vec![1, 2];
        let expected: bool = false;
        let actual = Solution::unique_occurrences(nums);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let nums: Vec<i32> = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        let expected: bool = true;
        let actual = Solution::unique_occurrences(nums);
        assert_eq!(expected, actual);
    }
}
