/// @author: Leon
/// https://leetcode.com/problems/subsets/
/// Time Complexity:    O(`len_n`!)
/// Space Complexity:   O(`len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len_n: usize = nums.len();
        let mut visited: Vec<bool> = vec![false; len_n];
        let mut path: Vec<i32> = Vec::new();
        let mut paths: Vec<Vec<i32>> = Vec::new();
        Self::backtrack(0, &mut visited, &nums, &mut path, &mut paths);
        paths
    }
    fn backtrack(
        idx_start: usize,
        visited: &mut Vec<bool>,
        nums: &Vec<i32>,
        path: &mut Vec<i32>,
        paths: &mut Vec<Vec<i32>>,
    ) {
        let len_n: usize = nums.len();
        if idx_start == len_n {
            return;
        }
        paths.push(path.to_vec());
        for idx in idx_start..len_n {
            if visited[idx] {
                continue;
            }
            visited[idx] = true;
            path.push(nums[idx]);
            Self::backtrack(idx, visited, nums, path, paths);
            visited[idx] = false;
            path.pop();
        }
    }
}
