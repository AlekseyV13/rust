fn is_palindrome(num: u32) -> bool {
    let original = num.to_string();
    let reversed: String = original.chars().rev().collect();
    original == reversed
}
#[test]
fn test11() {
    let number = 121;
    if is_palindrome(number) {
        println!("{} є паліндромом", number);
    } else {
        println!("{} не є паліндромом", number);
    }
}