/// https://leetcode.com/problems/largest-color-value-in-a-directed-graph/
/// 
/// Time Complexity:    O(V + E) ~ O(`len_c` + `len_e`)
/// Space Complexity:   O(V + E) ~ O(`len_c` + `len_e`)
/// 
/// Reference:
/// https://leetcode.com/problems/largest-color-value-in-a-directed-graph/discuss/1198658/C%2B%2B-Topological-Sort
use std::cmp::max;
use std::collections::{ HashSet, HashMap, VecDeque };

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let len_c: usize = colors.len();

        // not used
        // let len_e: usize = edges.len();
        
        let chs: Vec<char> = colors.chars().collect();
        
        let (graph, mut indegrees) = {
            let mut graph = HashMap::<u32, HashSet<u32>>::new();
            let mut indegrees: Vec<u32> = vec![0; len_c];
            
            for edge in edges.into_iter(){
                let u: u32 = edge[0] as u32;
                let v: u32 = edge[1] as u32;
                
                graph.entry(u).or_insert(HashSet::new()).insert(v);
                indegrees[v as usize] += 1;
            }
            
            (graph, indegrees)
        };
        
        let mut queue = VecDeque::<u32>::new();
        let mut ch_to_chs_freqs: Vec<Vec<u32>> = vec![vec![0; 26]; len_c];
        for (idx, &indegree) in indegrees.iter().enumerate(){
            if indegree == 0{
                queue.push_back(idx as u32);
                ch_to_chs_freqs[idx][chs[idx] as usize - 'a' as usize] = 1
            }
        }
        
        let mut seen: usize = 0;
        let mut largest: i32 = 0;
        
        while !queue.is_empty(){
            if let Some(cur) = queue.pop_front(){
                seen += 1;
                
                if let Some(&cur_max) = ch_to_chs_freqs[cur as usize].iter().max(){
                    largest = max(largest, cur_max as i32);
                }
                
                if let Some(next_vertices) = graph.get(&(cur as u32)){
                    for &next in next_vertices.iter(){
                        let next: usize = next as usize;
                        for i in 0..26{
                            ch_to_chs_freqs[next][i] = max(ch_to_chs_freqs[next][i], 
                                ch_to_chs_freqs[cur as usize][i] + if chs[next] as usize - 'a' as usize == i { 1 } else { 0 });
                        }

                        indegrees[next] -= 1;
                        if indegrees[next] == 0{
                            queue.push_back(next as u32);
                        }
                    }
                }
            }
        }
        
        if seen == len_c { largest } else { -1 }
    }
}