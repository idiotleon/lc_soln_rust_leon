/// @author: Leon
/// https://leetcode.com/problems/image-smoother/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(1) / O(`len_rs` * `len_cs`)
/// Referenece:
/// https://leetcode.com/problems/image-smoother/discuss/106602/Very-Clean-Solution-in-Java
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
        let len_rs: usize = img.len();
        let len_cs: usize = img[0].len();
        let mut ans: Vec<Vec<i32>> = vec![vec![0; len_cs]; len_rs];
        for r in 0..len_rs {
            for c in 0..len_cs {
                let mut sum: i32 = 0;
                let mut count: i32 = 0;
                for r_nxt in r as isize - 1..=r as isize + 1 {
                    for c_nxt in c as isize - 1..=c as isize + 1 {
                        if r_nxt < 0 || c_nxt < 0 {
                            continue;
                        }
                        let r_nxt: usize = r_nxt as usize;
                        let c_nxt: usize = c_nxt as usize;
                        if r_nxt >= len_rs || c_nxt >= len_cs {
                            continue;
                        }
                        sum += img[r_nxt][c_nxt];
                        count += 1;
                    }
                }
                ans[r][c] = sum / count;
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let img: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let actual = Solution::image_smoother(img);
        let expected = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_2() {
        let img: Vec<Vec<i32>> = vec![
            vec![2, 3, 4],
            vec![5, 6, 7],
            vec![8, 9, 10],
            vec![11, 12, 13],
            vec![14, 15, 16],
        ];
        let actual = Solution::image_smoother(img);
        let expected = vec![
            vec![4, 4, 5],
            vec![5, 6, 6],
            vec![8, 9, 9],
            vec![11, 12, 12],
            vec![13, 13, 14],
        ];
        assert_eq!(expected, actual);
    }
}
