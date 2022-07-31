use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// @author: Leon
/// https://leetcode.com/problems/lru-cache/
/// Time Complexities:
///         `new()`:    O(1)
///         `get()`:    O(1)
///         `put()`:    O(1)
/// Space Complexity:   O(`capacity`)
#[allow(dead_code)]
struct LRUCache {
    num_to_node: HashMap<i32, Rc<RefCell<ListNode>>>,
    dll: DoublyLinkedList,
    /// this is supposed to be a constant. However,
    /// there is no easy way to enforce immutability(constant/readonly) at fields level till now,
    /// or one has to rely on getters/setters
    capacity: usize,
}

#[allow(dead_code)]
impl LRUCache {
    fn new(capacity: i32) -> Self {
        let capacity: usize = capacity as usize;
        Self {
            /// the `HashMap::with_capacity()` rounds up to the nearest factor of 2
            /// e.g.
            /// capacity initialized    ->  actual capacity
            /// 2, 3                    ->  4
            /// 4                       ->  8
            /// 8                       ->  16
            num_to_node: HashMap::with_capacity(capacity + 1),
            dll: DoublyLinkedList::new(),
            capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.num_to_node.get(&key) {
            // to put the node at the head of DLL
            self.dll.remove(node.clone());
            self.dll.insert_at_head(node.clone());
            // to return the value
            return node.borrow().val;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.num_to_node.get(&key) {
            // to update the value in the DLL
            node.borrow_mut().val = value;
            // to move the node at head
            self.dll.remove(node.clone());
            self.dll.insert_at_head(node.clone());
        } else {
            // to create a new node
            let node: Rc<RefCell<ListNode>> = Rc::new(RefCell::new(ListNode::new(key, value)));
            // to insert the entry into the hashmap
            self.num_to_node.insert(key, node.clone());
            // to insert the new node into the DLL
            self.dll.insert_at_head(node.clone());
            // to trim the DLL and hashmap if there are more elements/caches than `capacity`
            if self.capacity < self.num_to_node.len() {
                let last = self.dll.get_last();
                let key = last.clone().borrow().key;
                self.num_to_node.remove(&key);
                self.dll.remove(last.clone());
            }
        }
    }
}

struct DoublyLinkedList {
    dummy_head: Rc<RefCell<ListNode>>,
    dummy_tail: Rc<RefCell<ListNode>>,
}

impl DoublyLinkedList {
    pub fn new() -> Self {
        let dummy_head: Rc<RefCell<ListNode>> = Rc::new(RefCell::new(ListNode::new(-1, -1)));
        let dummy_tail: Rc<RefCell<ListNode>> = Rc::new(RefCell::new(ListNode::new(-1, -1)));
        dummy_head.borrow_mut().next = Some(dummy_tail.clone());
        dummy_tail.borrow_mut().prev = Some(dummy_head.clone());
        Self {
            dummy_head,
            dummy_tail,
        }
    }
    /// according to
    /// 1. the current design
    /// 2. the description of the problem: capacity >= 1
    /// it is guaranteed that
    /// 1. there always is any ListNode before the `dummy_tail`
    /// 2. there always is at least one ListNodes, which is not the `dummy_head`, before the `dummy_tail`
    pub fn get_last(&self) -> Rc<RefCell<ListNode>> {
        self.dummy_tail.borrow().prev.clone().unwrap()
    }
    pub fn insert_at_head(&self, node: Rc<RefCell<ListNode>>) {
        let next = self.dummy_head.borrow().next.clone().unwrap();
        self.dummy_head.borrow_mut().next = Some(node.clone());
        node.borrow_mut().prev = Some(self.dummy_head.clone());
        node.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(node.clone());
    }
    pub fn remove(&self, node: Rc<RefCell<ListNode>>) -> Rc<RefCell<ListNode>> {
        let prev = node.borrow().prev.clone().unwrap();
        let next = node.borrow().next.clone().unwrap();
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev.clone());
        node
    }
}

struct ListNode {
    val: i32,
    key: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    pub fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            prev: None,
            next: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let mut lru: LRUCache = LRUCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        let expected0 = 1;
        let actual0 = lru.get(1);
        assert_eq!(expected0, actual0);
        lru.put(3, 3);
        let expected1 = -1;
        let actual1 = lru.get(2);
        assert_eq!(expected1, actual1);
        lru.put(4, 4);
        let expected2 = -1;
        let actual2 = lru.get(1);
        assert_eq!(expected2, actual2);
        let expected3 = lru.get(3);
        let actual3 = 3;
        assert_eq!(expected3, actual3);
        let expected4 = lru.get(4);
        let actual4 = 4;
        assert_eq!(expected4, actual4);
    }
    #[test]
    fn test_with_test_case_10_should_return_expected() {
        let mut lru: LRUCache = LRUCache::new(2);
        lru.put(1, 0);
        lru.put(2, 2);
        let expected0 = 0;
        let actual0 = lru.get(1);
        assert_eq!(expected0, actual0);
        lru.put(3, 3);
        let expected1 = -1;
        let actual1 = lru.get(2);
        assert_eq!(expected1, actual1);
        lru.put(4, 4);
        let expected2 = -1;
        let actual2 = lru.get(1);
        assert_eq!(expected2, actual2);
        let expected3 = lru.get(3);
        let actual3 = 3;
        assert_eq!(expected3, actual3);
        let expected4 = lru.get(4);
        let actual4 = 4;
        assert_eq!(expected4, actual4);
    }
    #[test]
    fn test_with_test_case_13_should_return_expected() {
        let mut lru: LRUCache = LRUCache::new(1);
        lru.put(2, 1);
        let expected0 = 1;
        let actual0 = lru.get(2);
        assert_eq!(expected0, actual0);
        lru.put(3, 2);
        let expected1 = -1;
        let actual1 = lru.get(2);
        assert_eq!(expected1, actual1);
        let expected2 = 2;
        let actual2 = lru.get(3);
        assert_eq!(expected2, actual2);
    }
}
