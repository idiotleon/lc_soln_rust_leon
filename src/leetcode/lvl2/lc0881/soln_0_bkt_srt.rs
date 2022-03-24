/// @author: Leon
/// https://leetcode.com/problems/boats-to-save-people/
/// Time Complexity:    O(`len_p`)
/// Space Complexity:   O(`limit`)
/// Reference:
/// https://leetcode.com/problems/boats-to-save-people/discuss/197063/easy-thought-process-to-improve-from-O(nlogn)-to-O(n)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let _len_p: usize = people.len();
        let limit: usize = limit as usize;
        let mut buckets: Vec<i16> = {
            let mut buckets: Vec<i16> = vec![0; limit + 1];
            for p in people{
                buckets[p as usize]+= 1;
            }
            buckets
        };
        let mut lo: usize = 0;
        let mut hi: usize = limit;
        let mut cnt: i32 = 0;
        while lo <= hi{
            while lo <= hi && buckets[lo] <= 0{
                lo += 1;
            }
            while lo <= hi && buckets[hi] <= 0{
                hi -= 1;
            }
            if lo > hi{
                break;
            }
            cnt += 1;
            if lo + hi <= limit{
                buckets[lo] -= 1;
            }
            buckets[hi] -= 1;
        }
        cnt
    }
}