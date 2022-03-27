//!输入一个整数数组,数组中只有一个数字出现了一次,而其他数字都出现了3次。请找出那个只出现一次的数字。例如,如果输入的数组为[0,1,0,1,0,1,100],则只出现一次的数字是100。
//! 扩展下 一个数字出现了m次 其他数字出现了n次。
#![allow(dead_code)]

use std::collections::HashMap;


/// 基础做法 循环求每个的 这个复杂度是O(nm)
fn solution(nums: Vec<u32>, m: u32, n: u32) -> u32 {
    // 如果m是奇数 n是偶数 那直接异或即可
    if m % 2 != 0 && n % 2 == 0 {
        return nums.iter().fold(0, |accum, item| accum ^ *item);
    }

    // 如果m不会被n整除 那么对他们的二进制位相加再mod n
    if m % n != 0 {
        let mut bit_sum = vec![0u32;32];
        for num in nums {
            for i in 1..=32 {
                bit_sum[i - 1] += (num >> (32 - i)) & 1;
            }
            println!("num : {num}. bit_sum: {bit_sum:?}");
        }

        let mut result : u32 = 0;
        for i in 0..=31 {
            result = (result << 1) + (if bit_sum[i] % n ==0 {0} else {1});
        }
        return result;
    }

    // 退回到最基础的解法
    let mut count_map = HashMap::<u32, u32>::new();
    for num in nums {
        let value = count_map.entry(num).or_default();
        *value = *value + 1;
    }
    for (k, v) in count_map.iter() {
        if *v == m {
            return *k;
        }
    }

    return 0;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(vec![100,2,2,3,3,200,200], 1, 2), 100);
        assert_eq!(solution(vec![0,1,0,1,0,1,100], 1, 3), 100);
        assert_eq!(solution(vec![0,1,0,1,0,1,100,100], 2, 3), 100);
        assert_eq!(solution(vec![0,1,0,1,100,100,100,100], 4, 2), 100);
    }

}
