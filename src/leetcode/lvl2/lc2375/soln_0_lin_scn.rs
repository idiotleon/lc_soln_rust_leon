/// @author: Leon
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let len_s: usize = pattern.len();
        const I: char = 'I';
        const D: char = 'D';
        let chs: Vec<char> = pattern.chars().collect();
        let mut ans: Vec<u8> = {
            let mut ans: Vec<u8> = vec![0; len_s + 1];
            for idx in 0..len_s + 1 {
                ans[idx] = idx as u8 + 1;
            }
            ans
        };
        let mut lo: usize = 0;
        while lo < len_s {
            if chs[lo] == D {
                let mut hi = lo + 1;
                while hi < len_s && chs[hi] == D {
                    hi += 1;
                }
                Self::reverse(lo, hi, &mut ans);
                lo = hi;
            } else {
                lo += 1;
            }
        }
        return ans
            .into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("");
    }
    fn reverse(lo: usize, hi: usize, vec: &mut Vec<u8>) {
        let mut lo: usize = lo;
        let mut hi: usize = hi;
        while lo < hi {
            vec.swap(lo, hi);
            lo += 1;
            hi -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let pattern: String = "IIIDIDDD".to_owned();
        let expected: String = "123549876".to_owned();
        let actual = Solution::smallest_number(pattern);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let pattern: String = "DDD".to_owned();
        let expected: String = "4321".to_owned();
        let actual = Solution::smallest_number(pattern);
        assert_eq!(expected, actual);
    }
}
