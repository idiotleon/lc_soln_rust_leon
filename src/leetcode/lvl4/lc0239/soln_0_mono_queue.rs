use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/sliding-window-maximum/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len_n: usize = nums.len();
        let k: usize = k as usize;
        let mut ans: Vec<i32> = Vec::with_capacity(len_n - k + 1);
        let mut deque: VecDeque<usize> = VecDeque::with_capacity(len_n - k + 1);
        for idx in 0..len_n{
            while let Some(&first) = deque.front(){
                if idx + 1 >= k && first < idx - k + 1{
                    deque.pop_front();
                }else{
                    break;
                }
            }
            while let Some(&last) = deque.back(){
                if nums[last] < nums[idx]{
                    deque.pop_back();
                }else{
                    break;
                }
            }
            deque.push_back(idx);
            if idx >= k - 1{
                ans.push(nums[*deque.front().unwrap()]);
            }
        }
        ans
    }
}