use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/non-decreasing-subsequences/
/// Time Complexity:    O(`len_ns` * (2 ^ `len_ns`))
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len_ns: usize = nums.len();
        let mut path: Vec<i32> = Vec::with_capacity(len_ns);
        let mut paths: Vec<Vec<i32>> = Vec::new();
        Self::backtrack(0, &mut path, &nums, &mut paths);
        return paths;
    }
    fn backtrack(
        idx_start: usize,
        path: &mut Vec<i32>,
        nums: &Vec<i32>,
        paths: &mut Vec<Vec<i32>>,
    ) {
        let len_ns: usize = nums.len();
        if path.len() > 1 {
            paths.push(path.to_vec());
        }
        let mut used: HashSet<i32> = HashSet::with_capacity(len_ns);
        for idx in idx_start..len_ns {
            if used.contains(&nums[idx]) {
                continue;
            }
            if path.is_empty() || nums[idx] >= *path.last().unwrap() {
                used.insert(nums[idx]);
                path.push(nums[idx]);
                Self::backtrack(1 + idx, path, nums, paths);
                path.pop();
            }
        }
    }
}
