/// @author: Leon
/// https://leetcode.com/problems/subsets-ii/
/// Time Complexity:    O(`_len_n` * (2 ^ `_len_n`)
/// Space Complexity:   O(`_len_n`)
/// Reference:
/// https://medium.com/@vasanths294/permutation-combination-subset-time-complexity-eca924e00071
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let _len_n = nums.len();
        let mut path: Vec<i32> = Vec::new();
        let mut paths: Vec<Vec<i32>> = Vec::new();
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        Self::backtrack(0, &mut path, &sorted, &mut paths);
        paths
    }
    fn backtrack(
        idx_start: usize,
        path: &mut Vec<i32>,
        nums: &Vec<i32>,
        paths: &mut Vec<Vec<i32>>,
    ) {
        let len_n = nums.len();
        paths.push(path.clone());
        for idx in idx_start..len_n {
            if idx != idx_start && nums[idx - 1] == nums[idx] {
                continue;
            }
            path.push(nums[idx]);
            Self::backtrack(idx + 1, path, nums, paths);
            path.pop();
        }
    }
}
