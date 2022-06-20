use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/employee-importance/
/// Time Complexity:    O(V + E) ~ O()
/// Space Complexity:   O(V + E) ~ O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_total_importance(employees: Vec<Employee>, id: i32) -> i32 {
        let _len_es: usize = employees.len();
        let (graph, id_to_imp) = Self::build_graph(employees);
        Self::dfs(id, &graph, &id_to_imp)
    }
    fn dfs(id: i32, graph: &HashMap<i32, Vec<i32>>, id_to_imp: &HashMap<i32, i32>) -> i32 {
        let mut sum: i32 = *id_to_imp.get(&id).unwrap();
        if let Some(nxts) = graph.get(&id) {
            for &nxt in nxts {
                sum += Self::dfs(nxt, graph, id_to_imp);
            }
            return sum;
        }
        0
    }
    fn build_graph(employees: Vec<Employee>) -> (HashMap<i32, Vec<i32>>, HashMap<i32, i32>) {
        let len_es: usize = employees.len();
        let mut id_to_subs: HashMap<i32, Vec<i32>> = HashMap::with_capacity(len_es);
        let mut id_to_imp: HashMap<i32, i32> = HashMap::with_capacity(len_es);
        for Employee {
            id,
            importance,
            subordinates,
        } in employees
        {
            id_to_subs.insert(id, subordinates);
            id_to_imp.insert(id, importance);
        }
        (id_to_subs, id_to_imp)
    }
}

struct Employee {
    id: i32,
    importance: i32,
    subordinates: Vec<i32>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1_should_return_expected() {
        let employees = vec![
            Employee {
                id: 1,
                importance: 5,
                subordinates: vec![2, 3],
            },
            Employee {
                id: 2,
                importance: 3,
                subordinates: vec![],
            },
            Employee {
                id: 3,
                importance: 3,
                subordinates: vec![],
            },
        ];
        let id: i32 = 1;
        let actual = Solution::get_total_importance(employees, id);
        let expected = 11;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_sample_input_2_should_return_expected() {
        let employees = vec![
            Employee {
                id: 1,
                importance: 2,
                subordinates: vec![5],
            },
            Employee {
                id: 5,
                importance: -3,
                subordinates: vec![],
            },
        ];
        let id: i32 = 5;
        let actual = Solution::get_total_importance(employees, id);
        let expected = -3;
        assert_eq!(expected, actual);
    }
}
