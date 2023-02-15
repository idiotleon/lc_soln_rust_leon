/// @author: Leon
/// https://leetcode.com/problems/add-to-array-form-of-integer/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_to_array_form(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len_ns: usize = nums.len();
        let mut ans: Vec<i32> = Vec::with_capacity(len_ns);
        let mut src = k;
        let mut carry: i32 = 0;
        let mut idx: isize = len_ns as isize - 1;
        while idx >= 0 || src > 0 || carry > 0 {
            let digit = src % 10;
            src /= 10;
            let sum = digit + carry + if idx >= 0 { nums[idx as usize] } else { 0 };
            idx -= 1;
            ans.push(sum % 10);
            carry = sum / 10;
        }
        ans.reverse();
        return ans;
    }
}
