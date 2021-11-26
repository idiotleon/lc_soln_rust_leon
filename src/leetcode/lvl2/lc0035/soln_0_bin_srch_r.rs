/// https://leetcode.com/problems/search-insert-position/
/// Time Complexity:    O(lg(`len_n`))
/// Space Complexity:   O(lg(`len_n`))
/// Reference:
/// https://leetcode.com/problems/search-insert-position/discuss/1596603/C%2B%2BPython-Simple-Solutions-w-Explanation-or-Recursive-%2B-Iterative-Binary-Search-and-Built-in
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len_n = nums.len();
        Self::search(0, len_n as isize - 1, &nums, target) as i32
    }
    fn search(lo: isize, hi: isize, nums: &Vec<i32>, target: i32) -> usize {
        if lo > hi {
            return lo as usize;
        }
        let mid = (lo + (hi - lo) / 2) as usize;
        if nums[mid] == target {
            return mid;
        }
        if nums[mid] < target {
            Self::search(mid as isize + 1, hi, nums, target)
        } else {
            Self::search(lo, mid as isize - 1, nums, target)
        }
    }
}
