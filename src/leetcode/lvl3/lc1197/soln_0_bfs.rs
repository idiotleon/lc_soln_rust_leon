/// @author: Leon
/// https://leetcode.com/problems/minimum-knight-moves/
/// 
/// Time Complexity:    O()
/// Space Complexity:   O()
/// 
/// Reference:
/// https://leetcode.com/problems/minimum-knight-moves/discuss/947138/Python-3-or-BFS-DFS-Math-or-Explanation
/// https://leetcode.com/problems/minimum-knight-moves/discuss/387071/JavaPython-3-BFS-code-using-symmetry
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_knight_moves(x: i32, y: i32) -> i32 {
        use std::collections::{HashSet, VecDeque};
        
        const DIRS: &[(i32, i32); 8] = &[(-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1), (1, 2), (2, 1)];
        
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        queue.push_back((0, 0));
        
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((0, 0));
        
        let mut moves: i32 = 0;
        
        while !queue.is_empty(){
            let len_q = queue.len();

            for _ in 0..len_q{
                if let Some((row, col)) = queue.pop_front(){
                    if row == x && col == y{
                        return moves;
                    }
                    
                    for (delta_r, delta_c) in DIRS{
                        let next = (row + delta_r, col + delta_c);
                        
                        if visited.insert(next){
                            queue.push_back(next);
                        }
                    }
                }
            }
            
            moves += 1;
        }
        -1
    }
}