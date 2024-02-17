/// @author: Leon
/// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/
/// Time Complexity:    O(lg(`len_n`))
/// Space Complexity:   O(`len_n`)
/// Reference:
/// http://zxi.mytechroad.com/blog/divide-and-conquer/leetcode-153-find-minimum-in-rotated-sorted-array/
/// https://www.youtube.com/watch?v=P4r7mF1Jd50
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let len_n = nums.len();
        Self::find(0, len_n - 1, &nums)
    }
    fn find(lo: usize, hi: usize, nums: &Vec<i32>) -> i32 {
        if lo + 1 >= hi {
            return std::cmp::min(nums[lo], nums[hi]);
        }
        if nums[lo] < nums[hi] {
            return nums[lo];
        }
        let mid: usize = lo + (hi - lo) / 2;
        std::cmp::min(Self::find(lo, mid, nums), Self::find(mid + 1, hi, nums))
    }
}
