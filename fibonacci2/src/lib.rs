


pub fn fibonacci(n: u32) -> u32 {
    if n == 0 || n==1 {
        return n;
    }
    return fibonacci(n-1)+fibonacci(n-2);
}