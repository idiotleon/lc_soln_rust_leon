/// @author: Leon
/// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/discuss/1676980/JavaPython-3-Sliding-Window-O(n)-code-w-brief-explanation-and-analysis.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        const ONE: i32 = 1;
        let cnt_ones: u16 = {
            let mut cnt: u16 = 0;
            for &num in nums.iter() {
                if num == ONE {
                    cnt += 1;
                }
            }
            cnt
        };
        // the maximum ones within the sliding window ever possible
        let mut max: u16 = 0;
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        // current counts of ones within the sliding window
        let mut win_ones: u16 = 0;
        while hi < len_n * 2 {
            win_ones += nums[hi % len_n] as u16;
            if hi - lo + 1 > cnt_ones as usize {
                win_ones -= nums[lo % len_n] as u16;
                lo += 1;
            }
            max = std::cmp::max(max, win_ones);
            hi += 1;
        }
        (cnt_ones - max) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums = vec![0, 1, 0, 1, 1, 0];
        let actual = Solution::min_swaps(nums);
        let expected = 1;
        assert_eq!(expected, actual);
    }
}
