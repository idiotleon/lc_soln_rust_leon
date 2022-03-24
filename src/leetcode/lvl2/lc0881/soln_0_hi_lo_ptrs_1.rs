/// @author: Leon
/// https://leetcode.com/problems/boats-to-save-people/
/// Time Complexity:    O(`len_p` * lg(`len_p`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let len_p: usize = people.len();
        let sorted: Vec<i32> = {
            let mut sorted = people;
            sorted.sort();
            sorted
        };
        let mut lo = 0;
        let mut hi = len_p - 1;
        let mut cnt: i32 = 0;
        while lo <= hi{
            cnt += 1;
            if hi == 0{
                break;
            }
            if sorted[lo] + sorted[hi] > limit{
                hi -= 1;
            }else{
                lo += 1;
                hi -= 1;
            }
        }
        cnt
    }
}