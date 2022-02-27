// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::hash::Hash;
// use std::rc::Rc;
/// @author: Leon
/// https://leetcode.com/problems/clone-graph/
/// Time Complexity:    O(V + E)
/// Space Complexity:   O(V + E)
struct Solution;

#[allow(dead_code)]
impl Solution {
    // fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    //     let mut edge: HashMap<Rc<Node>, Rc<RefCell<Node>>> = HashMap::new();
    //     Self::dfs(node, edge)
    // }
    // fn dfs(
    //     node: Option<Rc<RefCell<Node>>>,
    //     edge: HashMap<Rc<Node>, Rc<RefCell<Node>>>,
    // ) -> Option<Rc<RefCell<Node>>> {
    //     match node {
    //         Some(n) => {
    //             if let Some(e) = edge.get(n.clone()) {
    //                 return Some(e);
    //             }
    //             let value = n.borrow().value;
    //             let cloned = Rc::new(RefCell::new(Node::new(value)));
    //             edge.insert(n, cloned);
    //             for nei in n.borrow().neighbors {
    //                 cloned.borrow_mut().neighbors.push(Self::dfs(nei, edge));
    //             }
    //             Some(cloned)
    //         }
    //         None => None,
    //     }
    // }
}

// #[derive(PartialEq, Eq, Hash)]
// struct Node {
//     value: i32,
//     neighbors: Vec<Option<Rc<RefCell<Node>>>>,
// }

// impl Node {
//     fn new(value: i32) -> Node {
//         Node {
//             value,
//             neighbors: vec![],
//         }
//     }
// }

// impl Hash for Node {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.value.hash(state);
//         self.neighbors.hash(state);
//     }
// }
