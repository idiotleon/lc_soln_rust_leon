/// https://leetcode.com/problems/exclusive-time-olc0
/// 
/// Time Complexity:    O(`len_logs`)
/// Space Complexity:   O(`len_logs`)
/// 
/// Reference:
/// https://leetcode.com/problems/exclusive-time-of-functions/discuss/105062/Java-Stack-Solution-O(n)-Time-O(n)-Space
/// https://leetcode.com/problems/exclusive-time-of-functions/discuss/105062/Java-Stack-Solution-O(n)-Time-O(n)-Space/107796#[allow(dead_code)]
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        use std::collections::VecDeque;

        // not used
        // let len_logs = logs.len();
        
        const STATE_START: &str = "start";
        const SPLITTER: char = ':';
        
        let mut timeline = vec![0; n as usize];
        
        let mut stack = VecDeque::<i32>::new();
        let mut prev_ts = 0;
        
        for log in logs{
            let res: Vec<&str> = log.split(SPLITTER).collect();
            let id = res[0].to_string().parse::<i32>().unwrap();
            let state = res[1];
            let cur_ts = res[2].to_string().parse::<i32>().unwrap();
            
            if let Some(&idx_top) = stack.back(){
                timeline[idx_top as usize] += cur_ts - prev_ts;
            }
            
            prev_ts = cur_ts;
            
            if state == STATE_START.to_string(){
                stack.push_back(id)
            }else{
                timeline[stack.pop_back().unwrap() as usize] += 1;
                prev_ts += 1;
            }
        }
        
        
        timeline
    }
}