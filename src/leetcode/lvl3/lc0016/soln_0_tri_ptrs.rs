/// @author: leon
/// https://leetcode.com/problems/3sum-closest/
/// Time Complexity:    O(`len_ns` ^ 2)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/3sum-closest/discuss/7872/Java-solution-with-O(n2)-for-reference/9007
/// https://leetcode.com/problems/3sum-closest/discuss/7872/Java-solution-with-O(n2)-for-reference
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let len_ns = nums.len();
        let nums: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let mut closest = nums[0] + nums[1] + nums[len_ns - 1];
        for idx in 0..len_ns - 2 {
            let mut lo = idx + 1;
            let mut hi = len_ns - 1;
            while lo < hi {
                let sum = nums[idx] + nums[lo] + nums[hi];
                if (target - sum).abs() < (target - closest).abs() {
                    closest = sum;
                }
                if sum < target {
                    while lo < hi && nums[lo] == nums[lo + 1] {
                        lo += 1;
                    }
                    lo += 1;
                } else if sum > target {
                    while lo < hi && nums[hi - 1] == nums[hi] {
                        hi -= 1;
                    }
                    hi -= 1;
                } else {
                    return sum;
                }
            }
        }
        return closest;
    }
}
