///# Z自行变换
pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows < 2 {
        return s;
    }
    let num_rows = num_rows as usize;
    let mut vec = vec![String::from(""); num_rows];
    let chars = s.chars().collect::<Vec<_>>();
    let mut i = 0i32;
    let mut flag = -1;
    for c in chars {
        if i == 0 || i as usize == num_rows - 1 {
            flag = -flag;
        }
        vec[i as usize].push(c);
        i = i + flag;
    }
    vec.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let num_rows = 4;
        let s = "PAYPALISHIRING".to_string();

        assert_eq!("PINALSIGYAHRPI", convert(s, num_rows));
    }
    #[test]
    fn test_2() {
        let num_rows = 4;
        let s = "A".to_string();

        assert_eq!("A", convert(s, num_rows));
    }

    #[test]
    fn test_3() {
        let num_rows = 1;
        let s = "A".to_string();

        assert_eq!("A", convert(s, num_rows));
    }

    #[test]
    fn test_4() {
        let num_rows = 3;
        let s = "PAYPALISHIRING".to_string();

        assert_eq!("PAHNAPLSIIGYIR", convert(s, num_rows));
    }
}
