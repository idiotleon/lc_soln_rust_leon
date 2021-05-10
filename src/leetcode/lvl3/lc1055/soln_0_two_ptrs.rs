/// https://leetcode.com/problems/shortest-way-to-form-string/
/// 
/// Time Complexity:    O(`len_s` * `len_t`)
/// Space Complexity:   O(`len_s`) + O(`len_t`) ~ O(`len_s`)
///
/// Reference:
/// https://leetcode.com/problems/shortest-way-to-form-string/discuss/304193/Java-Two-Pointers-Solution-With-Explanation
/// https://leetcode.com/problems/shortest-way-to-form-string/discuss/330938/Accept-is-not-enough-to-get-a-hire.-Interviewee-4-follow-up
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_way(source: String, target: String) -> i32 {
        // not used
        // let len_s = source.len();
        let len_t = target.len();
        
        let chs_s: Vec<char> = source.chars().collect();
        let chs_t: Vec<char> = target.chars().collect();
        
        let mut shortest = 0;
        
        let mut idx_t = 0;
        while idx_t < len_t{
            let prev_idx_t = idx_t;
            
            for (idx_s, &ch) in chs_s.iter().enumerate(){
                if idx_t < len_t && chs_s[idx_s] == chs_t[idx_t]{
                    idx_t += 1;
                }
            }
            
            if prev_idx_t == idx_t{
                return -1;
            }
            
            shortest += 1;
        }
        
        shortest
    }
}