/// @author: Leon
/// https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len_ns: usize = nums.len();
        let mut freqs: Vec<usize> = vec![0; len_ns + 1];
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(len_ns);
        for num in nums {
            let freq = &freqs[num as usize];
            if *freq >= ans.len() {
                ans.push(Vec::new());
            }
            ans[*freq].push(num);
            freqs[num as usize] += 1;
        }
        return ans;
    }
}
