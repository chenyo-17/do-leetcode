#![allow(dead_code)]

use std::cmp::max;
use std::collections::HashMap;

type Cache = HashMap<i32, i32>;

/// Return the maximum subsequence sum of the `arr[:right+1]`
/// Not used in `main`, just for reference
fn max_subseq_sum(nums: &Vec<i32>, right: i32, cache: &mut Cache) -> i32 {
    if let Some(&v) = cache.get(&right) {
        return v;
    }
    if right == 0 {
        let sum_max = max(0, nums[0]);
        cache.insert(right, sum_max);
        return sum_max;
    }
    if right < 0 {
        return 0;
    }

    let sub_max = max_subseq_sum(nums, right - 1, cache);
    let sum_max = max(sub_max + nums[right as usize], sub_max);
    cache.insert(right, sum_max);
    sum_max
}

/// Return the maximum subsequence sum of the `arr[:right+1]` with no adjacent elements
fn max_subseq_sum_no_adjacent(nums: &Vec<i32>, right: i32, cache: &mut Cache) -> i32 {
    if let Some(&v) = cache.get(&right) {
        return v;
    }
    if right == 0 {
        let sum_max = max(0, nums[0]);
        cache.insert(right, sum_max);
        return sum_max;
    }
    if right < 0 {
        return 0;
    }

    // either include the `right` element, then cannot include `right-1`
    // or exclude the `right` element
    let sum_1 = max_subseq_sum_no_adjacent(nums, right - 2, cache) + nums[right as usize];
    let sum_2 = max_subseq_sum_no_adjacent(nums, right - 1, cache);

    let sum_max = max(sum_1, sum_2);
    cache.insert(right, sum_max);
    sum_max
}

/// Change `nums[query[0]]` to `query[1]`
fn modify_nums(nums: &mut Vec<i32>, query: &Vec<i32>) {
    let (idx, val) = (query[0] as usize, query[1]);
    nums[idx] = val;
}

/// For each query, modify the `nums` array and
/// calculate the maximum subsequence sum with no adjacent elements
/// Return the sum of all maximum subsequence sums
fn solution(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    // for each point in `queries`, modify the `nums` array
    // then calculate the maximum subsequence sum with no adjacent elements
    let mut res = 0;
    let mut nums = nums;
    let mut cache = Cache::new();
    for query in queries {
        // delete key >= query[0] from cache, they are no longer valid
        cache.retain(|&k, _| k < query[0]);
        modify_nums(&mut nums, &query);
        let max_sum = max_subseq_sum_no_adjacent(&nums, nums.len() as i32 - 1, &mut cache);
        res += max_sum;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subseq_sum() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let mut cache = Cache::new();
        let max_sum = max_subseq_sum(&nums, nums.len() as i32 - 1, &mut cache);
        assert_eq!(max_sum, 12);
    }

    #[test]
    fn test_max_subseq_sum_no_adjacent() {
        let nums = vec![3, 2, 5, 20, 7];
        let mut cache = Cache::new();
        let max_sum = max_subseq_sum_no_adjacent(&nums, nums.len() as i32 - 1, &mut cache);
        assert_eq!(max_sum, 23);
    }

    #[test]
    fn test_modify_nums() {
        let mut nums = vec![3, 2, 5, 20, 7];
        let query = vec![2, 10];
        modify_nums(&mut nums, &query);
        assert_eq!(nums, vec![3, 2, 10, 20, 7]);
    }

    #[test]
    fn test_solution_1() {
        let nums = vec![3, 5, 9];
        let queries = vec![vec![1, -2], vec![0, -3]];
        let res = solution(nums, queries);
        assert_eq!(res, 21);
    }

    #[test]
    fn test_solution_2() {
        let nums = vec![0, -1];
        let queries = vec![vec![0, -5]];
        let res = solution(nums, queries);
        assert_eq!(res, 0);
    }
}
