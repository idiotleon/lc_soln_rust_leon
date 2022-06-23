use std::collections::BinaryHeap;
use std::cmp::Reverse;
/// @author: Leon
/// https://leetcode.com/problems/course-schedule-iii/
/// Time Complexity:    O(`len_n` * lg(`len_n`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let len_cs: usize = courses.len();
        let sorted: Vec<Vec<i32>> = {
            let mut courses = courses;
            courses.sort_by_key(|e| e[1]);
            courses
        };
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        // the running/overrall duration
        let mut sum: i32 = 0;
        for course in sorted{
            let duration = course[0];
            let last_day = course[1];
            heap.push(Reverse(duration));
            sum += duration;
            if sum > last_day{
                if let Some(Reverse(top)) = heap.pop(){
                    sum -= top;
                }
            }
        }
        heap.len() as i32
    }
}

#[cfg()]