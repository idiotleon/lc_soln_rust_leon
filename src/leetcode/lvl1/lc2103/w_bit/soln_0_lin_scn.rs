/// @author: Leon
/// https://leetcode.com/problems/rings-and-rods/
/// Time Complexity:    O(`RANGE`)
/// Space Complexity:   O(`len_r`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_points(rings: String) -> i32 {
        const RANGE: usize = 100;
        let len_r: usize = rings.len();
        let rod_to_ring: Vec<u8> = {
            let mut res: Vec<u8> = vec![0; RANGE];
            let chs: Vec<char> = rings.chars().collect();
            for idx_chs in (0..len_r).step_by(2) {
                let idx: usize = chs[idx_chs + 1] as usize - '0' as usize;
                res[idx] = res[idx] | (1 << Self::get_hash(chs[idx_chs]));
            }
            res
        };
        let cnt: u8 = {
            let mut cnt: u8 = 0;
            for freq in rod_to_ring {
                if freq == 0b111 {
                    cnt += 1;
                }
            }
            cnt
        };
        cnt as i32
    }
    fn get_hash(ch: char) -> u8 {
        match ch {
            'B' => 0,
            'G' => 1,
            'R' => 2,
            _ => unreachable!(),
        }
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
