/// @author: Leon
/// https://leetcode.com/problems/minimize-product-sum-of-two-arrays/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_product_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        const RANGE: usize = 100 + 1;
        let mut freqs1: Vec<i32> = vec![0; RANGE];
        let mut freqs2: Vec<i32> = vec![0; RANGE];
        for (idx, num1) in nums1.into_iter().enumerate(){
            freqs1[num1 as usize] += 1;
            freqs2[nums2[idx] as usize] += 1;
        }
        let mut lo: usize = 1;
        let mut hi: usize = RANGE - 1;
        let mut ans: i32 = 0;
        while lo < RANGE && hi > 0{
            while lo < RANGE && freqs1[lo] == 0{
                lo += 1;
            }
            while hi > 0 && freqs2[hi] == 0{
                hi -= 1;
            }
            ans += (lo * hi) as i32;
            if lo == RANGE || hi == 0{
                return ans;
            }
            freqs1[lo] -= 1;
            freqs2[hi] -= 1;
        }
        ans
    }
}