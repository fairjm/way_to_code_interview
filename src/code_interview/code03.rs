//!输入一个非负数n，请计算0到n之间每个数字的二进制形式中1的个数，并输出一个数组。例如，输入的n为4，由于0、1、2、3、4的二进制形式中1的个数分别为0、1、1、2、1，因此输出数组[0，1，1，2，1]。
#![allow(dead_code)]
/// 计算1的数量 x & (x - 1)
fn count1(num: u32) -> u32 {
    let mut n = num;
    let mut count = 0;
    while n != 0 {
        count = count + 1;
        n = n & (n - 1);
    }
    count
}

/// 右移
fn count1_2(num: u32) -> u32 {
    let mut n = num;
    let mut count = 0;
    while n != 0 {
        count = count + 1;
        n = n >> 1;
    }
    count
}

/// 基础做法 循环求每个的 这个复杂度是O(nm)
fn solution1(num: u32) -> Vec<u32> {
    let mut result = Vec::new();
    for i in 0..=num {
        result.push(count1(i));
    }
    result
}

/// 从count1的方式可以知道 x 会比 x & (x - 1)多一个1 可以用这个方式去得到
/// 并且0的1数量为0
fn solution2(num: u32) -> Vec<u32> {
    let n = num as usize;
    let mut result = vec![0; n + 1];
    for i in 1..=n {
        result[i] = result[i & (i - 1)] + 1;
    }
    result
}

/// 另一个思路 偶数右移一位和原先的1数量相同 但奇数会少1位(因为奇数末尾是1)
fn solution3(num: u32) -> Vec<u32> {
    let n = num as usize;
    let mut result = vec![0 as u32; n + 1];
    for i in 1..=n {
        result[i] = result[i / 2] + (i as u32 & 1);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count1() {
        assert_eq!(count1(0), 0);
        assert_eq!(count1(1), 1);
        assert_eq!(count1(2), 1);
        assert_eq!(count1(3), 2);
        assert_eq!(count1(4), 1);
        assert_eq!(count1(5), 2);
        assert_eq!(count1(6), 2);
        assert_eq!(count1(7), 3);
    }

    #[test]
    fn test_count1_2() {
        assert_eq!(count1(0), 0);
        assert_eq!(count1(1), 1);
        assert_eq!(count1(2), 1);
        assert_eq!(count1(3), 2);
        assert_eq!(count1(4), 1);
        assert_eq!(count1(5), 2);
        assert_eq!(count1(6), 2);
        assert_eq!(count1(7), 3);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solution1(4), vec![0, 1, 1, 2, 1]);
        assert_eq!(solution2(4), vec![0, 1, 1, 2, 1]);
        assert_eq!(solution3(4), vec![0, 1, 1, 2, 1]);
    }
}
