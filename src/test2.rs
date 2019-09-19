pub fn fib(n: usize) -> usize {
    if n == 0||n == 1{
        return 1
    }else{
        let s = fib(n-1)+fib(n-2);
        return s
    }
}

pub fn fib_sum() -> usize {
    let mut s = 0;
    let mut n = 0;
    while fib(n) < 4000000{
        if fib(n) % 2 == 0 {
            s += fib(n);
        }
        n+=1;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test2() {
        assert_eq!(4613732,fib_sum());
    }

}