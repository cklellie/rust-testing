fn mutable() -> i32 {
    let mut x = 5;
    print_x(x);
    x = 6;
    print_x(x);
    return x;
}

#[test]
fn mutable_test() {
    assert_eq!(mutable(), 6)
}

fn print_x(x: i32) {
    println!("The value of x is: {}", x);
}