fn main() {
    simple_if_else();
    multiple_if_else();
    if_in_a_let();
}

fn simple_if_else() {
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn multiple_if_else() {
    let number = 11;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2.");
    }
}

fn if_in_a_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

#[test]
fn return_value_from_loops(){
    let mut counter =0;
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };
    println!("The result is {result}");
    assert_eq!(result, 20);
}

#[test]
fn labeled_loops(){
    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    assert_eq!(count, 2);
}

#[test]
fn while_loops(){
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!");
    assert_eq!(number, 0);
}

#[test]
fn while_looping_collection(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    assert_eq!(index, 5);
}

#[test]
fn for_looping_collection(){
    let a = [10, 20, 30, 40, 50];
    let mut i = 0;
    for element in a {
        println!("the value is: {element}");
        assert_eq!(element, a[i]);
        i += 1;
    }
}

fn for_loop_with_range(){
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!")
}

#[test]
fn for_loop_with_collection_iterator(){
    let a = [1,2,3,4,5];

    for number in a.iter().rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!")
}