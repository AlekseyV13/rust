fn test31() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

#[test]
fn test_test31_positive(){
    test31();
}

fn test32(){
    let mut x = 1;
    x = x + 2;

    assert_eq!(x, 3);
    println!("Success2");
}

#[test]
fn test_test32_positive(){
    test32();
}

fn test33() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}


#[test]
fn test_test33_positive(){
    test33();
}

fn test34() {
    define_x()
}
fn define_x() {
    let x = "hello";

    println!("{}, world", x)
}

#[test]
fn test_test34_positive(){
    test34();
}

fn test35() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

#[test]
fn test_test35_positive(){
    test35();
}

fn test36() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    x = x + 3;

    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}

#[test]
fn test_test36_positive(){
    test36();
}

#[allow(unused_variables)]
fn test37() {
    let x = 1;
}

// Warning: unused variable: `x`

#[test]
fn test_test37_positive(){
    test37();
}

fn test38() {
    let  (mut x, y) = (1, 2);
    x = x + 2;


    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success8!");
}

#[test]
fn test_test38_positive(){
    test38();
}

fn test39() {
    let ( x, y);

    (x,..) = (3, 4);
    [..,y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success9!");
}

#[test]
fn test_test39_positive(){
    test39();
}

