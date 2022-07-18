/// @author: Leon
/// https://leetcode.com/problems/intersection-of-multiple-arrays/
/// Time Complexity:    O(`len_a` * `_len_ns`)
/// Space Complexity:   O(`RANGE`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn intersection(arr: Vec<Vec<i32>>) -> Vec<i32> {
        let len_a: u16 = arr.len() as u16;
        let _len_ns: usize = arr[0].len();
        const RANGE: usize = 1e3 as usize + 7;
        let freqs: Vec<u16> = {
            let mut freqs: Vec<u16> = vec![0; RANGE];
            for nums in arr {
                for num in nums {
                    freqs[num as usize] += 1;
                }
            }
            freqs
        };
        freqs
            .into_iter()
            .enumerate()
            .filter(|(_num, freq)| *freq == len_a)
            .map(|(num, _freq)| num as i32)
            .collect::<Vec<i32>>()
    }
}
