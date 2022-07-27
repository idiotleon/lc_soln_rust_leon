/// @author: Leon
/// https://leetcode.com/problems/find-closest-number-to-zero/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let _len_ns: usize = nums.len();
        const RANGE: i32 = 1e5 as i32 + 7;
        let mut ans: i32 = RANGE - 0;
        let mut closest: i32 = RANGE - 0;
        for num in nums {
            let diff = (num - 0).abs();
            if diff < closest {
                closest = diff;
                ans = num;
            } else if diff == closest {
                if ans < num {
                    ans = num;
                }
            }
        }
        ans
    }
}
