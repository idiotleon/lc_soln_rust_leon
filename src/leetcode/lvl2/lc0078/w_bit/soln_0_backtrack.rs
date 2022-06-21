/// @author: Leon
/// https://leetcode.com/problems/subsets/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len_n: usize = nums.len();
        let range = 1 << len_n;
        let mut paths: Vec<Vec<i32>> = Vec::new();
        for bitmask in 0..range {
            let mut path: Vec<i32> = Vec::new();
            for idx in 0..len_n {
                if bitmask & (1 << idx) != 0 {
                    path.push(nums[idx]);
                }
            }
            paths.push(path.to_vec());
        }
        paths
    }
}
