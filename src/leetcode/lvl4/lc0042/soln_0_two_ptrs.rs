/// https://leetcode.com/problems/trapping-rain-water/
/// 
/// Time Complexity:    O(`len`)
/// Space Complexity:   O(1)
/// 
/// Reference:
/// https://leetcode.com/problems/trapping-rain-water/discuss/17391/Share-my-short-solution./185869
/// https://leetcode.com/problems/trapping-rain-water/discuss/17391/Share-my-short-solution.
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn trap(heights: Vec<i32>) -> i32 {
        let len: usize = heights.len();
        
        let mut lo: i32 = 0;
        let mut hi: i32 = len as i32 - 1;
        let mut lo_max: i32 = 0;
        let mut hi_max: i32 = 0;
        
        let mut water: i32 = 0;
        
        while lo <= hi{
            lo_max = std::cmp::max(lo_max, heights[lo as usize]);
            hi_max = std::cmp::max(hi_max, heights[hi as usize]);
            
            if lo_max < hi_max{
                water += lo_max - heights[lo as usize];
                lo += 1;
            }else{
                water += hi_max - heights[hi as usize];
                hi -= 1;
            }
        }
        
        water
    }
}