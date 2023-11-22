use std::collections::VecDeque;

/// @author: Leon
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        const DIRS: &[isize] = &[0, -1, 0, 1, 0];
        let len_rs: usize = image.len();
        let len_cs: usize = image[0].len();
        let sr: usize = sr as usize;
        let sc: usize = sc as usize;
        let original: i32 = image[sr][sc];
        if original == color {
            return image;
        }
        let mut queue: VecDeque<(usize, usize)> = {
            let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(len_rs * len_cs);
            queue.push_back((sr, sc));
            queue
        };
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some((r, c)) = queue.pop_front() {
                    image[r][c] = color;
                    for d in 0..4 {
                        let r_nxt: isize = r as isize + DIRS[d];
                        let c_nxt: isize = c as isize + DIRS[d + 1];
                        if r_nxt < 0 || c_nxt < 0 {
                            continue;
                        }
                        let r_nxt: usize = r_nxt as usize;
                        let c_nxt: usize = c_nxt as usize;
                        if r_nxt >= len_rs || c_nxt >= len_cs || image[r_nxt][c_nxt] != original {
                            continue;
                        }
                        queue.push_back((r_nxt, c_nxt));
                    }
                }
            }
        }
        return image;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let image: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let sr: i32 = 1;
        let sc: i32 = 1;
        let color: i32 = 2;
        let expected: Vec<Vec<i32>> = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        let actual: Vec<Vec<i32>> = Solution::flood_fill(image, sr, sc, color);
        assert_eq!(expected, actual);
    }
}
