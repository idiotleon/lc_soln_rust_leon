/// @author: Leon
/// https://leetcode.com/problems/flatten-2d-vector/
/// Time Complexity:
///         `new()`:        O(`len`)
///         `next()`:       O(1)
///         `has_next()`:   O(1)
/// Space Complexity:       O(`len`)
#[allow(dead_code)]
struct Vector2D {
    nums: Vec<i32>,
    idx: usize,
}

#[allow(dead_code)]
impl Vector2D {
    fn new(vec: Vec<Vec<i32>>) -> Self {
        let len_rs: usize = vec.len();
        let len: usize = vec.iter().map(|r| r.len()).sum();
        let mut nums: Vec<i32> = Vec::with_capacity(len);
        for r in 0..len_rs {
            for c in 0..vec[r].len() {
                nums.push(vec[r][c]);
            }
        }
        Self { nums, idx: 0 }
    }

    fn next(&mut self) -> i32 {
        let num = self.nums[self.idx];
        self.idx += 1;
        num
    }

    fn has_next(&self) -> bool {
        self.idx < self.nums.len()
    }
}
