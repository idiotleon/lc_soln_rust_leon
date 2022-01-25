/// @author: Leon
/// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
/// Time Complexity:    O(lg(`len_n`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let len_n: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut sorted = nums.to_vec();
            sorted.sort();
            sorted
        };
        let mut ans: Vec<i32> = vec![0; len_n];
        for (idx, num) in nums.into_iter().enumerate() {
            ans[idx] = 1 + Self::higher_bound(num - 1, &sorted) as i32;
        }
        ans
    }
    fn higher_bound(target: i32, sorted: &Vec<i32>) -> usize {
        let len_n: usize = sorted.len();
        let mut lo: isize = 0;
        let mut hi: isize = len_n as isize;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if sorted[mid as usize] <= target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        hi as usize - 1
    }
}
