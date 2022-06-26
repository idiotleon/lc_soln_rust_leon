/// @author: Leon
/// https://leetcode.com/problems/maximum-xor-after-operations/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/maximum-xor-after-operations/discuss/2196294/Thought-process-explained-oror-Basic-intuition-to-optimise
/// https://leetcode.com/problems/maximum-xor-after-operations/discuss/2195929/JavaC%2B%2BPython-1-Line-Solution-OR-of-All-Elements
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        let _len_n: usize = nums.len();
        let mut ans: i32 = 0;
        for num in nums {
            ans |= num;
        }
        ans
    }
}
