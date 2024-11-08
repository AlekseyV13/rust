fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
#[test]
fn test9() {
    let number = 29;
    if is_prime(number) {
        println!("{} є простим числом", number);
    } else {
        println!("{} не є простим числом", number);
    }
}
