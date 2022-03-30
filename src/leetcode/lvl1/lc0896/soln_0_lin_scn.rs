/// @author: Leon
/// https://leetcode.com/problems/monotonic-array/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let len_n: usize = nums.len();
        // the flag indicating if it is increasing
        let mut is_inc: Option<bool> = None;
        for idx in 1..len_n {
            let diff: i32 = nums[idx] - nums[idx - 1];
            if diff > 0 {
                match is_inc {
                    Some(ii) => {
                        if !ii {
                            return false;
                        }
                    }
                    None => is_inc = Some(true),
                }
            } else if diff < 0 {
                match is_inc {
                    Some(ii) => {
                        if ii {
                            return false;
                        }
                    }
                    None => is_inc = Some(false),
                }
            }
        }
        true
    }
}
