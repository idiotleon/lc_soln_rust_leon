use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/rings-and-rods/
/// Time Complexity:    O(`RANGE`)
/// Space Complexity:   O(`len_r`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_points(rings: String) -> i32 {
        const RANGE: usize = 100 + 7;
        let len_r: usize = rings.len();
        let rod_to_ring: Vec<HashSet<char>> = {
            let mut rod_to_ring: Vec<HashSet<char>> = vec![HashSet::new(); RANGE];
            let chs: Vec<char> = rings.chars().collect();
            for idx in (0..len_r).step_by(2) {
                rod_to_ring[chs[idx + 1] as usize - '0' as usize].insert(chs[idx]);
            }
            rod_to_ring
        };
        let mut cnt: u8 = 0;
        for set in rod_to_ring.into_iter() {
            if set.len() == 3 {
                cnt += 1;
            }
        }
        cnt as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let rings = "B0B6G0R6R0R6G9".to_owned();
        let actual = Solution::count_points(rings);
        let expected = 1;
        assert_eq!(expected, actual);
    }
}
