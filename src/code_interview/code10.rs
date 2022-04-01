//! 题目：输入一个整数数组和一个整数k，请问数组中有多少个数字之和等于k的连续子数组？
//! 例如，输入数组[1，1，1]，k的值为2，有2个连续子数组之和等于2。
//! 此题因为没有说明是正整数 所以不适用双索引
#![allow(dead_code)]

use std::collections::HashMap;

fn solution(nums: Vec<i32>, k: i32) -> u32 {
    let mut sum_map = HashMap::new();
    // 这个是表示取0-i
    sum_map.insert(0, 1);
    let mut count: u32 = 0;
    let mut sum = 0;
    for i in 0..nums.len() {
        sum = sum + nums[i];
        count = count + *sum_map.entry(sum - k).or_insert(0);
        let entry = sum_map.entry(sum).or_default();
        *entry = *entry + 1;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(vec![1,1,1], 2), 2);
        assert_eq!(solution(vec![1,1,1,2], 2), 3);
    }

}
