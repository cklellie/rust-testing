fn guess() -> u8 {
    let guess: u8 = "42".parse().expect("Not a number!");
    println!("guess is {}", guess);
    return guess;
}

#[test]
fn guess_test() {
    assert_eq!(guess(), 42);
}

#[test]
fn integer_literals_test() {
    let decimal = 98_222;
    assert_eq!(decimal, 98222);
    let hex = 0xff;
    assert_eq!(hex, 255);
    let octal = 0o77;
    assert_eq!(octal, 63);
    let binary = 0b1111_0000;
    assert_eq!(binary, 240);
    let byte = b'A';
    assert_eq!(byte, 65);
}

#[test]
fn floating_test() {
    let x = 2.0;
    let y: f32 = 2.0;
    assert_eq!(x, y);
}

#[test]
fn numberic_operations_test(){
    let sum = 5 + 10;
    assert_eq!(sum, 15);
    let difference = 95.5 - 4.3;
    assert_eq!(difference, 91.2);
    let product = 4 * 30;
    assert_eq!(product, 120);
    let quotient = 56.7/ 32.2;
    assert_eq!(quotient, 1.7608695652173911);
    let whole = 3 / 2;
    assert_eq!(whole, 1);
    let remainder = 43 % 5;
    assert_eq!(remainder, 3);
}

#[test]
fn booleans(){
    let t = true;
    assert!(t);
    let f: bool = false;
    assert_ne!(t, f);
}

#[test]
fn characters() {
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    let all_together_now = format!("{}{}{}", c, z, heart_eyed_cat );
    assert_eq!(all_together_now, "zZ😻")
}