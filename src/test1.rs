
pub fn fiz_sum(n:usize) -> usize {
    let mut sum = 0;
    for i in 0..n {
        if i % 3 == 0 {
            sum += i;
        }else if i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test1() {
        assert_eq!(233168,fiz_sum(1000));
    }

}