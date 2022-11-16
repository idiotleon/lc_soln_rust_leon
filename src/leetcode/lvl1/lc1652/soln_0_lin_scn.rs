/// @author: Leon
/// https://leetcode.com/problems/defuse-the-bomb/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn decrypt(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len_ns: usize = nums.len();
        let mut ans: Vec<i32> = vec![0; len_ns];
        if k == 0 {
            return ans;
        }
        let is_negative: bool = k < 0;
        let k: usize = if k > 0 { k as usize } else { -k as usize };
        if is_negative {
            nums.reverse();
        }
        let mut sum: i32 = 0;
        for idx in 1..=k as usize {
            sum += nums[idx % len_ns];
        }
        ans[0] = sum;
        for idx in 1..len_ns {
            sum -= nums[idx];
            sum += nums[(idx + k) % len_ns];
            ans[idx] = sum;
        }
        if is_negative {
            ans.reverse();
        }
        return ans;
    }
}
