//!输入一个数组,如何找出数组中所有和为0的3个数字的三元组？需要注意的是,返回值中不得包含重复的三元组。
//! 例如,在数组[-1,0,1,2,-1,-4]中有两个三元组的和为0,它们分别是[-1,0,1]和[-1,-1,2]。
//! 其实就是threeSum
#![allow(dead_code)]

fn solution(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let len = nums.len();
    if  len >= 3{
        nums.sort();
        let mut i = 0;
        while i < len - 2 {
            two_sum(i, &nums, &mut result);
            let tmp = nums[i];
            while i < len - 2 && tmp == nums[i] {
                i = i + 1;
            } 
        } 
    }
    result
}

fn two_sum(i: usize, nums: &Vec<i32>, result: &mut Vec<Vec<i32>>) {
    let num_i = nums[i];
    let mut left = i + 1;
    let mut right = nums.len() - 1;
    while left < right {
        let num_left = nums[left];
        let num_right = nums[right];
        let sum = num_i + num_left + num_right;
        if sum == 0 {
            result.push(vec![num_i, num_left, num_right]);
            while left < right && num_left == nums[left] {
                left = left + 1;
            }
        } else if sum > 0 {
            right = right -1;
        } else {
            left = left + 1;
        }

    } 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(vec![-1,0,1,2,-1,-4]), vec![vec![-1,-1,2], vec![-1,0,1]]);
    }

}
