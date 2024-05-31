#![allow(dead_code)]

struct Solution {}
pub fn main() {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut nums1_id: usize = 0;
        let mut nums2_id: usize = 0;
        // used to check whether we have visited all nums1
        let mut nums1_visited: usize = 0;
        while nums1_id < m as usize + n as usize && nums2_id < n as usize {
            // if current nums2 is no bigger than current nums1,
            // move forward nums1, until the current nums1 is bigger than nums2.
            if nums1_visited == m as usize {
                // have visited all nums1
                break;
            }
            if nums1[nums1_id] <= nums2[nums2_id] {
                nums1_id += 1;
                nums1_visited += 1;
            } else {
                // insert current nums2 at nums1_id
                nums1.insert(nums1_id, nums2[nums2_id]);
                // drop the last 0
                nums1.truncate(nums1.len() - 1);
                nums2_id += 1;
                nums1_id += 1;
            }
        }
        // if there are unvisited nums2, append it in the nums1 end
        while nums2_id < n as usize {
            nums1[nums1_id] = nums2[nums2_id];
            nums1_id += 1;
            nums2_id += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_solution_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_solution_3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }
}
