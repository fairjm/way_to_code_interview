//!可以画一下找下规律 2(n-1)是两个完整列每行之间的差距 比如3行 需要下两行上两行
//!然后用这个就可以得到中间的余数关系

#[allow(dead_code)]
pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }   
    let words:Vec<char> = s.chars().collect();
    let len = words.len() as i32;
    let mut new_words = vec![String::new();num_rows as usize];
    let turn = 2 * num_rows -2;
    for i in 0..len {
        let mut m = i % turn;
        if m >= num_rows {
            m = turn - m;
        }
        (*(new_words.get_mut(m as usize).unwrap())).push(words[i as usize]);
    }
    new_words.join("")
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count1() {
        assert_eq!(convert("PAYPALISHIRING".to_owned(),3), "PAHNAPLSIIGYIR".to_owned());
        assert_eq!(convert("PAYPALISHIRING".to_owned(),4), "PINALSIGYAHRPI".to_owned());
        assert_eq!(convert("A".to_owned(),1), "A".to_owned());
    }
}