pub fn find_max(n:usize) -> usize {
    let (mut k,mut n,mut x) = (0, n ,2);
    while n > 1 {
        if n % x == 0 {
            n /= x;
            k = x;
            x = 2;
        }else{
            x += 1;
        }
    }
    k
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test3() {
        assert_eq!(6857,find_max(600851475143));
    }

}