/// @author: Leon
/// https://leetcode.com/problems/3sum/
/// Time Complexity:    O(`len_n` ^ 2)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len_n = nums.len();
        let mut ans = Vec::<Vec<i32>>::new();
        if len_n < 3 {
            return ans;
        }
        let sorted = {
            let mut tmp = nums;
            tmp.sort();
            tmp
        };
        for idx in 0..len_n - 2 {
            if idx > 0 && sorted[idx - 1] == sorted[idx] {
                continue;
            }
            let mut lo = idx + 1;
            let mut hi = len_n - 1;
            let target = -sorted[idx];
            while lo < hi {
                let sum = sorted[lo] + sorted[hi];
                if sum == target {
                    ans.push(vec![sorted[idx], sorted[lo], sorted[hi]]);
                    lo += 1;
                    hi -= 1;
                    while lo < hi && sorted[lo - 1] == sorted[lo] {
                        lo += 1;
                    }
                    while lo < hi && sorted[hi] == sorted[hi + 1] {
                        hi -= 1;
                    }
                } else if sum > target {
                    hi -= 1;
                } else {
                    lo += 1;
                }
            }
        }
        ans
    }
}
