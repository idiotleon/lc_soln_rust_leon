/// https://leetcode.com/problems/execution-of-all-suffix-instructions-staying-in-a-grid/
/// Time Complexity:    O(`len_s` ^ 2)
/// Space Complexity:   O(1) / O(`len_s`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    const RIGHT: char = 'R';
    const LEFT: char = 'L';
    const UP: char = 'U';
    const DOWN: char = 'D';
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let ans: Vec<i32> = {
            let mut ans: Vec<i32> = vec![0; len_s];
            let mut idx: usize = 0;
            while idx < len_s {
                ans[idx] = Self::execute(idx, n, &start_pos, &chs);
                idx += 1;
            }
            ans
        };
        ans
    }
    fn execute(idx_ch: usize, n: i32, start_po: &Vec<i32>, chs: &Vec<char>) -> i32 {
        let len_s: usize = chs.len();
        let n: isize = n as isize;
        let mut idx: usize = idx_ch as usize;
        let mut steps = 0;
        let mut r: isize = start_po[0] as isize;
        let mut c: isize = start_po[1] as isize;
        while idx < len_s {
            match chs[idx] {
                Self::RIGHT => c += 1,
                Self::LEFT => c -= 1,
                Self::UP => r -= 1,
                Self::DOWN => r += 1,
                _ => unreachable!(),
            }
            if r < 0 || r >= n || c < 0 || c >= n {
                return steps;
            }
            idx += 1;
            steps += 1;
        }
        steps
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample_input_1() {
        let n = 3;
        let start_pos = vec![0, 1];
        let s = "RRDDLU".to_owned();
        let actual = Solution::execute_instructions(n, start_pos, s);
        let expected = vec![1, 5, 4, 3, 1, 0];
        assert_eq!(expected, actual);
    }
}
