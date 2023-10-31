/// @author: Leon
/// https://leetcode.com/problems/find-the-original-array-of-prefix-xor/
/// Time Complexity:    O(`len_ps`)
/// Space Complexity:   O(`len_ps`)
/// Reference:
/// https://leetcode.com/problems/find-the-original-array-of-prefix-xor/editorial/?envType=daily-question&envId=2023-10-31
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let len_ps: usize = pref.len();
        let mut nums: Vec<i32> = vec![0; len_ps];
        nums[0] = pref[0];
        for idx in 1..len_ps {
            nums[idx] = pref[idx] ^ pref[idx - 1];
        }
        return nums;
    }
}
