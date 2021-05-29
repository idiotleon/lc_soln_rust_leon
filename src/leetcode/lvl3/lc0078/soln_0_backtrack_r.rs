/// https://leetcode.com/problems/subsets/
///
/// Time Complexity:    O(`len_n`!)
/// Space Complexity:   O(`len_n`)
///
/// Reference:
/// https://leetcode.com/problems/subsets/discuss/27281/A-general-approach-to-backtracking-questions-in-Java-(Subsets-Permutations-Combination-Sum-Palindrome-Partitioning)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut path: Vec<i32> = Vec::new();
        let mut paths: Vec<Vec<i32>> = Vec::new();

        Self::backtrack(0, &nums, &mut path, &mut paths);

        paths
    }

    fn backtrack(
        idx_start: usize,
        nums: &Vec<i32>,
        path: &mut Vec<i32>,
        paths: &mut Vec<Vec<i32>>,
    ) {
        let len_n: usize = nums.len();

        paths.push(path.clone());

        for idx in idx_start..len_n {
            path.push(nums[idx]);
            Self::backtrack(idx + 1, nums, path, paths);
            path.pop();
        }
    }
}
