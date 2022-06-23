/// @author: Leon
/// https://leetcode.com/problems/4sum/
/// Time Complexity:    O(`len_n` ^ 3)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len_n: usize = nums.len();
        if len_n < 4 {
            return vec![];
        };
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let sorted: Vec<i32> = {
            let mut tmp: Vec<i32> = nums;
            tmp.sort();
            tmp
        };
        for idx1 in 0..len_n - 3 {
            if idx1 > 0 && sorted[idx1 - 1] == sorted[idx1] {
                continue;
            }
            for idx2 in idx1 + 1..len_n - 2 {
                if idx2 > idx1 + 1 && sorted[idx2 - 1] == sorted[idx2] {
                    continue;
                }
                let mut lo: usize = idx2 + 1;
                let mut hi: usize = len_n - 1;
                while lo < hi {
                    let sum = sorted[idx1] + sorted[idx2] + sorted[lo] + sorted[hi];
                    if sum == target {
                        ans.push(vec![sorted[idx1], sorted[idx2], sorted[lo], sorted[hi]]);
                        lo += 1;
                        hi -= 1;
                        while lo < hi && sorted[lo - 1] == sorted[lo] {
                            lo += 1;
                        }
                        while lo < hi && sorted[hi] == sorted[hi + 1] {
                            hi -= 1;
                        }
                    } else if sum < target {
                        lo += 1;
                    } else {
                        hi -= 1;
                    }
                }
            }
        }
        ans
    }
}
