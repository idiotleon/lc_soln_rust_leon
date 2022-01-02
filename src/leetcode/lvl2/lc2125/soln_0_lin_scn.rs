/// @author: Leon
/// https://leetcode.com/problems/number-of-laser-beams-in-a-bank/
/// Time Complexity:    O(`len_b` * len_str)
/// Space Complexity:   O(`len_b`) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let len_b = bank.len();
        const ONE: char = '1';
        // to count frequencies of ones
        let cnts_one: Vec<u16> = {
            let mut res: Vec<u16> = vec![0; len_b];
            let mut idx: usize = 0;
            for b in bank {
                let mut cnt: u16 = 0;
                for ch in b.chars() {
                    if ch == ONE {
                        cnt += 1;
                    }
                }
                res[idx] = cnt;
                idx += 1;
            }
            res
        };
        // to calculate the result
        let mut cnt: i32 = 0;
        let mut lo: usize = 0;
        while lo < len_b && cnts_one[lo] == 0 {
            lo += 1;
        }
        let mut hi: usize = lo + 1;
        while hi < len_b {
            while hi < len_b && cnts_one[hi] == 0 {
                hi += 1;
            }
            if hi >= len_b {
                break;
            }
            cnt += cnts_one[lo] as i32 * cnts_one[hi] as i32;
            lo = hi;
            hi += 1;
        }
        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let bank = vec![
            "011001".to_owned(),
            "000000".to_owned(),
            "010100".to_owned(),
            "001000".to_owned(),
        ];
        let actual = Solution::number_of_beams(bank);
        let expected = 8;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let bank = vec!["000".to_owned(), "111".to_owned(), "000".to_owned()];
        let actual = Solution::number_of_beams(bank);
        let expected = 0;
        assert_eq!(expected, actual);
    }
}
