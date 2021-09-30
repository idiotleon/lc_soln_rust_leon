/// https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/
///
/// Time Complexity:    O(`right`)
/// Space Complexity:   O(`RNAGE_DATA`)
///
/// Reference:
/// https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/discuss/1267923/Line-Sweep-O(n-%2B-m)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        const RANGE_DATA: usize = 50 + 7;

        let line: Vec<u8> = {
            let mut tmp = vec![0; RANGE_DATA];
            for range in ranges {
                let start: usize = range[0] as usize;
                let end: usize = (range[1] + 1) as usize;

                tmp[start] += 1;
                tmp[end] -= 1;
            }
            tmp
        };

        let mut overlaps = 0;
        for idx in 1..=right as usize {
            overlaps += line[idx];

            if idx >= left as usize && overlaps == 0 {
                return false;
            }
        }

        true
    }
}
