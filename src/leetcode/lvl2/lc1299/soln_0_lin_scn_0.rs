/// https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/
/// Time Complexity:    O(`_len_n`d)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/discuss/466005/Concise-Rust-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn replace_elements(mut nums: Vec<i32>) -> Vec<i32> {
        let _len_n: usize = nums.len();
        let mut max: i32 = -1;
        for cur in nums.iter_mut().rev(){
            let prev = *cur;
            *cur = max;
            max = std::cmp::max(max, prev);
        }
        nums
    }
}