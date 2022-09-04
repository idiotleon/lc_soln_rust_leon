struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        const IMPL: i32 = -1;
        let _len_s: usize = s.len();
        let mut indices: Vec<i32> = vec![IMPL; 26];
        for (idx, ch) in s.chars().enumerate() {
            let idx_ch: usize = ch as usize - 'a' as usize;
            if indices[idx_ch] == IMPL {
                indices[idx_ch] = idx as i32;
            } else {
                if (idx as i32 - indices[idx_ch] - 1).abs() != distance[idx_ch] {
                    return false;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1_should_return_expected() {
        let s: String = "abaccb".to_owned();
        let distance: Vec<i32> = vec![
            1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let expected: bool = true;
        let actual = Solution::check_distances(s, distance);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_sample_input_2_should_return_expected() {
        let s: String = "aa".to_owned();
        let distance: Vec<i32> = vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let expected: bool = false;
        let actual = Solution::check_distances(s, distance);
        assert_eq!(expected, actual);
    }
}
