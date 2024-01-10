fn shadowing() -> i32 {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    print_x(x);
    return x;
}

#[test]
fn shadowing_test() {
    assert_eq!(shadowing(), 12)
}

fn print_x(x: i32) {
    println!("The value of x is: {}", x);
}