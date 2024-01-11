fn main() {
    println!("Hello, world!");

    another_function();
    another_function_2(5);
    print_label_measurement(5, 'h');
    weird_return_syntax_function();
    print_returned();
    print_checks();
}

fn print_label_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_2(x: i32) {
    println!("Another function with a parameter: {x}");
}

fn weird_return_syntax_function(){
    let y:i32 = {
        let x = 3;
        x+1
    };
    println!("{y}");
}

fn return_without_return_keyword() -> i32 {
    5
}

fn return_with_return_keyword() -> i32 {
    return 6
}

fn return_with_return_keyword_and_semicolon() -> i32 {
    return 7;
}

fn print_returned() {
    println!("{}", return_without_return_keyword());
    println!("{}", return_with_return_keyword());
    println!("{}", return_with_return_keyword_and_semicolon());
}

fn print_checks(){
    //unindexed, count must match gives in order.
    println!("{}, {}", return_with_return_keyword(), return_with_return_keyword_and_semicolon() );

    //indexed, repeated
    println!( "{0}, {1}, {0}", return_without_return_keyword(), return_with_return_keyword_and_semicolon() );
}