fn main() {}

#[test]
fn add() {
    let sum = 5 + 10;
    assert_eq!(sum, 15)
}

#[test]
fn subtract() {
    let difference = 95.5 - 4.3;
    assert_eq!(difference, 91.2)
}

#[test]
fn multiplication() {
    let product = 4 * 30;
    assert_eq!(product, 120)
}

#[test]
fn division() {
    let quotient = 56.7 / 32.2;
    assert_eq!(quotient > 1.76, true);
    assert_eq!(quotient < 1.77, true);

    let floored = 2 / 3;
    assert_eq!(floored, 0);

    let whole = 4 / 2;
    assert_eq!(whole, 2);
}

#[test]
fn boolean() {
    let t = true;
    assert_eq!(t, true);

    let f = false;
    assert_ne!(f, true);
}

#[test]
fn character(){
    let c = 'Z';
    assert_eq!(c, 'Z');

    let z: char = 'Z';
    assert_eq!(c, z);
}

#[test]
fn tuple(){
    let tup = (500, "test", 'Z', true );
    let (x, y, z, a) = tup;
    assert_eq!(x, 500);
    assert_eq!(y, "test");
    assert_eq!(z, 'Z');
    assert_eq!(a, true);

    assert_eq!(x, tup.0);
    assert_eq!(y, tup.1);
    assert_eq!(z, tup.2);
    assert_eq!(a, tup.3);

    //check mutability of deconstructed.
    //use _ to disgard otherwise receive warnings.
    let (mut b, _c, _d, _e) = tup;
    b = b+1;
    assert_ne!(tup.0, b);
    assert_eq!(tup.0, 500);
}

#[test]
fn arrays(){
    let a: [i32; 5];
    //initialise array with 3s as default value.
    a = [3;5];

    assert_eq!(a, [3,3,3,3,3]);

    let b = [1,2,3,4,5];
    assert_eq!(b[0], 1);
    assert_eq!(b.len(), 5);
}
