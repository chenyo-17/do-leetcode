#![allow(dead_code)]

struct Solution {}
pub fn main() {}

/// Use a segment tree to store the obstacle ranges.
struct TreeNode {
    range: (usize, usize),
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // construct an empty tree root
    pub fn empty_root() -> Self {
        TreeNode {
            range: (0, std::usize::MAX),
            left: None,
            right: None,
        }
    }

    // Check if the node is a leaf node
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    // check whether the value is covered by the range
    fn cover_val(&self, val: usize) -> bool {
        self.range.0 <= val && val <= self.range.1
    }

    // Insert an obstacle value into the tree
    pub fn insert(&mut self, val: usize) {
        // find the leaf node to insert the value
        if self.is_leaf() {
            // insert the value
            let left = Self {
                range: (self.range.0, val),
                left: None,
                right: None,
            };
            let right = Self {
                range: (val, self.range.1),
                left: None,
                right: None,
            };
            self.left = Some(Box::new(left));
            self.right = Some(Box::new(right));
            return;
        }
        if self.cover_val(val) {
            // recursively insert the value to the left or right child
            if self.left.as_ref().unwrap().cover_val(val) {
                self.left.as_mut().unwrap().insert(val);
            } else {
                self.right.as_mut().unwrap().insert(val);
            }
            return;
        }
    }
}

// An iterator to traverse the leaf nodes of the tree
struct TreeNodeIter<'a> {
    stack: Vec<&'a TreeNode>,
}

impl<'a> TreeNodeIter<'a> {
    // Create a new iterator with the root node
    fn new(root: &'a TreeNode) -> Self {
        let mut stack = Vec::new();
        stack.push(root);
        Self { stack }
    }
}

impl Iterator for TreeNodeIter<'_> {
    type Item = (usize, usize);

    // The traversal is a DFS order
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            if node.is_leaf() {
                return Some(node.range);
            }
            if let Some(right) = &node.right {
                self.stack.push(right);
            }
            if let Some(left) = &node.left {
                self.stack.push(left);
            }
        }
        None
    }
}

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut res = Vec::new();
        let mut root = TreeNode::empty_root();
        for query in queries {
            let query_type = query[0];
            match query_type {
                1 => {
                    root.insert(query[1] as usize);
                }
                2 => {
                    let mut iter = TreeNodeIter::new(&root);
                    let mut found = false;
                    while let Some((start, end)) = iter.next() {
                        // reach the first obstacle that is behind query[1]
                        if end > query[1] as usize {
                            if query[1] as usize - start >= query[2] as usize {
                                found = true;
                                break;
                            }
                            break;
                        }
                        if end - start >= query[2] as usize {
                            found = true;
                            break;
                        }
                    }
                    res.push(found);
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
    fn test_insert() {
        let mut root = TreeNode::empty_root();
        root.insert(7);
        root.insert(2);
        let root_left = root.left.as_ref().unwrap();
        assert_eq!(root_left.range, (0, 7));
        assert_eq!(root.right.as_ref().unwrap().range, (7, std::usize::MAX));
        assert_eq!(root_left.left.as_ref().unwrap().range, (0, 2));
        assert_eq!(root_left.right.as_ref().unwrap().range, (2, 7));
    }

    #[test]
    fn test_iterator() {
        let mut root = TreeNode::empty_root();
        root.insert(7);
        root.insert(2);
        let mut iter = TreeNodeIter::new(&root);
        assert_eq!(iter.next(), Some((0, 2)));
        assert_eq!(iter.next(), Some((2, 7)));
        assert_eq!(iter.next(), Some((7, std::usize::MAX)));
        assert_eq!(iter.next(), None);
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
