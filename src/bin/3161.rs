#![allow(dead_code)]

use std::cmp::max;
use std::cmp::min;

struct Solution {}
pub fn main() {}

/// Use a segment tree to maintain the maximum block length
/// can be placed in a range.
/// E.g., if the range is [0, 5], then `max_len` stores the
/// maximum block length that starts at 1, 2,..5.
#[derive(Clone)]
struct TreeNode {
    range: (usize, usize),
    max_len: usize,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // construct a new segment tree for the given max range `MAX_RANGE`
    // no obstacle is inserted, so `max_len = MAX_RANGE` for all nodes
    pub fn construct(left: usize, right: usize, max_range: usize) -> Self {
        // reach the leaf node
        if left == right {
            return Self {
                range: (left, right),
                max_len: max_range - left,
                left: None,
                right: None,
            };
        }
        let mid = (left + right) / 2;
        let left_c = Self::construct(left, mid, max_range);
        let right_c = Self::construct(mid + 1, right, max_range);
        Self {
            range: (left, right),
            max_len: max(left_c.max_len, right_c.max_len),
            left: Some(Box::new(left_c)),
            right: Some(Box::new(right_c)),
        }
    }

    // whether the node is a leaf node
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    // Insert an obstacle value and update the tree
    // this should be called on the root node
    pub fn insert_obstacle(&mut self, pos: usize) {
        // base case: reach the leaf node
        if self.range.0 == self.range.1 {
            // if the point is before the obstacle,
            // it may need to update the `max_len`
            if self.range.0 < pos {
                self.max_len = min(self.max_len, pos - self.range.0);
            }
            return;
        }
        // if the obstacle is behind `self.range.0`,
        // it may need to update the `max_len`
        if pos > self.range.0 {
            // recursively update child nodes
            if pos > self.left.as_ref().unwrap().range.0 {
                // update the left child node
                self.left.as_mut().unwrap().insert_obstacle(pos);
            }
            if pos > self.right.as_ref().unwrap().range.0 {
                // update the right child node
                self.right.as_mut().unwrap().insert_obstacle(pos);
            }
            // update the `max_len` of the current node
            self.max_len = max(
                self.left.as_ref().unwrap().max_len,
                self.right.as_ref().unwrap().max_len,
            );
        }
    }

    /// Whether the block of size `sz` can be placed at position between 0 and `x`
    /// This should be called on the root node
    pub fn query(&self, x: usize, sz: usize) -> bool {
        // virtually insert the obstacle at `x`
        // TODO: this is expensive!
        let mut new_root = self.clone();
        new_root.insert_obstacle(x);
        new_root._query(x, sz)
    }

    fn _query(&self, x: usize, sz: usize) -> bool {
        // base case: the current range is before x
        if self.range.1 < x {
            return self.max_len >= sz;
        }
        if self.range.0 >= x {
            // cannot be placed
            return false;
        }
        // if the current range covers x, then recursively query the child nodes
        self.left.as_ref().unwrap().query(x, sz) || self.right.as_ref().unwrap().query(x, sz)
    }
}

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut res = Vec::new();
        let max_range = 5 * 10_i32.pow(4) as usize;
        let mut root = TreeNode::construct(0, max_range, max_range);
        for query in queries {
            let query_type = query[0];
            match query_type {
                1 => {
                    root.insert_obstacle(query[1] as usize);
                }
                2 => {
                    let x = query[1] as usize;
                    let sz = query[2] as usize;
                    res.push(root.query(x, sz));
                }
                _ => {}
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct() {
        let max_range = 10;
        let r = TreeNode::construct(0, max_range, max_range);
        assert_eq!(r.max_len, max_range);
        let l = r.left.as_ref().unwrap();
        let r = r.right.as_ref().unwrap();
        assert_eq!(l.max_len, max_range);
        assert_eq!(r.max_len, 4);
        let ll = l.left.as_ref().unwrap();
        let lr = l.right.as_ref().unwrap();
        assert_eq!(ll.max_len, max_range);
        assert_eq!(lr.max_len, 7);
        let rl = r.left.as_ref().unwrap();
        let rr = r.right.as_ref().unwrap();
        assert_eq!(rl.max_len, 4);
        assert_eq!(rr.max_len, 1);
        let lll = ll.left.as_ref().unwrap();
        let llr = ll.right.as_ref().unwrap();
        assert_eq!(lll.max_len, 10);
        assert_eq!(llr.max_len, 8);
        let lrl = lr.left.as_ref().unwrap();
        let lrr = lr.right.as_ref().unwrap();
        assert_eq!(lrl.max_len, 7);
        assert_eq!(lrr.max_len, 5);
        let rll = rl.left.as_ref().unwrap();
        let rlr = rl.right.as_ref().unwrap();
        assert_eq!(rll.max_len, 4);
        assert_eq!(rlr.max_len, 2);
    }

    #[test]
    fn test_insert() {
        let max_range = 10;
        let mut root = TreeNode::construct(0, max_range, max_range);
        root.insert_obstacle(7);
        assert_eq!(root.max_len, 7);
        let l = root.left.as_ref().unwrap();
        let r = root.right.as_ref().unwrap();
        assert_eq!(l.max_len, 7);
        assert_eq!(r.max_len, 3);
        let ll = l.left.as_ref().unwrap();
        let lr = l.right.as_ref().unwrap();
        assert_eq!(ll.max_len, 7);
        assert_eq!(lr.max_len, 4);
        let rl = r.left.as_ref().unwrap();
        let rr = r.right.as_ref().unwrap();
        assert_eq!(rl.max_len, 3);
        assert_eq!(rr.max_len, 1);
        let lll = ll.left.as_ref().unwrap();
        let llr = ll.right.as_ref().unwrap();
        assert_eq!(lll.max_len, 7);
        assert_eq!(llr.max_len, 5);
        let rll = rl.left.as_ref().unwrap();
        let rlr = rl.right.as_ref().unwrap();
        assert_eq!(rll.max_len, 3);
        assert_eq!(rlr.max_len, 2);
        let rlll = rll.left.as_ref().unwrap();
        let rllr = rll.right.as_ref().unwrap();
        assert_eq!(rlll.max_len, 1);
        assert_eq!(rllr.max_len, 3);
        root.insert_obstacle(2);
        assert_eq!(root.max_len, 5);
        let l = root.left.as_ref().unwrap();
        let r = root.right.as_ref().unwrap();
        assert_eq!(l.max_len, 5);
        assert_eq!(r.max_len, 3);
    }

    #[test]
    fn test_query() {
        let max_range = 10;
        let mut r = TreeNode::construct(0, max_range, max_range);
        r.insert_obstacle(7);
        assert_eq!(r.query(5, 4), true);
        assert_eq!(r.query(10, 8), false);
        r.insert_obstacle(2);
        assert_eq!(r.query(5, 4), false);
        assert_eq!(r.query(6, 3), true);
    }

    #[test]
    fn test_solution_1() {
        let queries = vec![vec![1, 2], vec![2, 3, 3], vec![2, 3, 1], vec![2, 2, 2]];
        let res = Solution::get_results(queries);
        assert_eq!(res, vec![false, true, true]);
    }

    #[test]
    fn test_solution_2() {
        let queries = vec![
            vec![1, 7],
            vec![2, 7, 6],
            vec![1, 2],
            vec![2, 7, 5],
            vec![2, 7, 6],
        ];
        let res = Solution::get_results(queries);
        assert_eq!(res, vec![true, true, false]);
    }
}
