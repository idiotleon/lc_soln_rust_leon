use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/find-original-array-from-doubled-array/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
/// Reference:
/// https://leetcode.com/problems/find-original-array-from-doubled-array/discuss/1470959/JavaC++Python-Match-from-the-Smallest-or-Biggest-100/1110603
///
/// This is NOT a correct solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_original_array(nums: Vec<i32>) -> Vec<i32> {
        let len_n: usize = nums.len();
        let mut ans: Vec<i32> = Vec::with_capacity(len_n / 2);
        if len_n % 2 == 1 {
            return ans;
        }
        let mut num_to_freq: HashMap<i32, u16> = {
            let mut map: HashMap<i32, u16> = HashMap::new();
            for num in nums {
                *map.entry(num).or_default() += 1;
            }
            map
        };
        let sorted: Vec<i32> = {
            let mut sorted: Vec<i32> = num_to_freq.keys().cloned().collect();
            sorted.sort();
            sorted
        };
        for num in sorted {
            if num_to_freq.get(&num).unwrap() > num_to_freq.get(&(num + num)).unwrap_or(&0) {
                return Vec::new();
            }
            for _ in 0..*num_to_freq.get(&num).unwrap_or(&0) {
                ans.push(num);
                *num_to_freq.entry(num + num).or_default() -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_test_case_152() {
        let nums: Vec<i32> = vec![0, 0, 0, 0];
        let actual = Solution::find_original_array(nums);
        let expected = vec![0, 0];
        assert_eq!(expected, actual);
    }
}
