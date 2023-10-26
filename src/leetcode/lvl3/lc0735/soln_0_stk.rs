use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/asteroid-collision/description/
/// Time Complexity:    O(`len_as`)
/// Space Complexity:   O(`len_as`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let len_as: usize = asteroids.len();
        let mut stk: VecDeque<i32> = VecDeque::with_capacity(len_as);
        for asteroid in asteroids {
            if stk.is_empty() || *stk.back().unwrap() < 0 || asteroid > 0 {
                stk.push_back(asteroid);
                continue;
            }
            while let Some(&top) = stk.back() {
                if top > 0 && top < -asteroid {
                    stk.pop_back();
                } else {
                    break;
                }
            }
            if let Some(&top) = stk.back() {
                if top < 0 {
                    stk.push_back(asteroid);
                } else if top == -asteroid {
                    stk.pop_back();
                }
            } else {
                stk.push_back(asteroid);
            }
        }
        return stk.into_iter().collect::<Vec<i32>>();
    }
}
