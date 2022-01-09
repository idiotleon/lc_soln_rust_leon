/// https://leetcode.com/problems/robot-bounded-in-circle/
/// Time Complexity:    O(`_len_instr`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/robot-bounded-in-circle/discuss/290856/JavaC%2B%2BPython-Let-Chopper-Help-Explain
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let _len_instr = instructions.len();
        const DIRS: &[(isize, isize)] = &[(0, -1), (-1, 0), (0, 1), (1, 0)];
        const STRT: char = 'G';
        const LEFT: char = 'L';
        const RIGHT: char = 'R';
        let mut row: isize = 0;
        let mut col: isize = 0;
        let mut d: usize = 0;
        for ch in instructions.chars() {
            match ch {
                STRT => {
                    row += DIRS[d].0;
                    col += DIRS[d].1;
                }
                LEFT => {
                    d = (d + 1) % 4;
                }
                RIGHT => {
                    d = (d + 3) % 4;
                }
                _ => unreachable!(),
            }
        }
        row == 0 && col == 0 || d > 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let instructions = "GGLLGG".to_owned();
        let actual = Solution::is_robot_bounded(instructions);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
