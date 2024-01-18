fn main() {
    let s1 = String::from("hello");
    // This:
    // let s2 = s1;
    // is a compilation error as heap based objects can only have one owner.
    // s1 goes out of scope after the "move" operation.
    // It is not a copy, but a move. If you want a "deep" copy, use the clone
    // method, though be sure that the performance overhead is acceptable.
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);


    // On the contrast, stack based objects do not have these semantics.
    // It works as a copy, as they implement the Copy trait.
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");

    takes_ownership(s);

    // not possible as s is no longer valid here.
    //println!("{}",s);

    let x = 5;
    makes_copy(x);

    println!("{}", x);

    return_values_scope();

    tuple_example();

    references_tuple_example();

    mutable_references_example();

    mutable_and_immutable_references();
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn return_values_scope() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("s1 = {}, s3 = {}. But s2 is invalid still.", s1, s3);
}

fn gives_ownership() -> String {
    let s1 = String::from("yours");
    return s1;
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}

fn tuple_example() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of {} is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    return (s, len);
}

fn references_tuple_example() {
    let s1 = String::from("hi there");
    let len = ref_calculate_length(&s1);
    println!("The lenth of {} is {}.", s1, len);
}

fn ref_calculate_length(s: &String) -> usize {

    //Atempting to change a borrowed (referenced) variable is not possible as it is not owned.
    //s.push_str("something more");

    return s.len();
}

fn mutable_references_example() {
    let mut s = String::from("another thing to change");

    change(&mut s);

    println!("s = {}", s);
}

fn change(s: &mut String) {
    s.push_str(" is now changed.");
}

fn mutable_and_immutable_references() {
    let mut s = String::from("Hi there.");

    let r1 = &s;
    let r2 = &s;

    //This print cannot be after r3, as it would result in compile error as there
    //would be an immutable and mutable reference in scope. After this print though
    // r1 and r2 are no longer used and considered out of scope, so it is possilbe
    // to create a mutable reference.
    println!("{} and {}", r1, r2);

    let r3 = &mut s;

    println!("{}", r3);
}

#[test]
fn string_slices() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string);

    assert_eq!(word, "hello");

    let another = "Some other sentence.";

    //Notice i don't need a reference `&`, as string literals are already a reference to a full slice.
    let first = first_word(another);

    assert_eq!(first, "Some");

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

#[test]
fn array_slice(){
    let a = [1,2,3,4,5];
    let slice = &a[1..3];

    assert_eq!(slice, [2,3]);
}