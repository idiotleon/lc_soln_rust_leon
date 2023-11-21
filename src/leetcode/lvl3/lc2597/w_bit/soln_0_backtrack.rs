/// @author: Leon
/// https://leetcode.com/problems/the-number-of-beautiful-subsets/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const RANGE_LEN: usize = 20;
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let len_ns: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let mut cnt: i32 = 0;
        for bit_mask in 1..(1 << len_ns) {
            if Self::is_eligible(&sorted, bit_mask, k) {
                cnt += 1;
            }
        }
        return cnt;
    }
    fn is_eligible(nums: &Vec<i32>, bit_mask: u32, k: i32) -> bool {
        let len_ns: usize = nums.len();
        for lo in 0..len_ns - 1 {
            if bit_mask & (1 << lo) > 0 {
                for hi in lo + 1..len_ns {
                    if bit_mask & (1 << hi) > 0 {
                        if (nums[lo] - nums[hi]).abs() == k {
                            return false;
                        }
                    }
                }
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
        let nums: Vec<i32> = vec![2, 4, 6];
        let k: i32 = 2;
        let expected: i32 = 4;
        let actual: i32 = Solution::beautiful_subsets(nums, k);
        assert_eq!(expected, actual);
    }
}
