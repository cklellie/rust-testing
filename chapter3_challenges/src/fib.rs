fn fib(n: i32) -> i128 {
    if n >= 20 {
        panic!("Recursive fib too slow!")
    }
    if n <= 0 {
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}

fn fib_non_recursive(n: i32) -> i128{
    if n >= 185 {
        panic!( "Number too large for i128");
    }
    if n <= 0{
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    }

    let mut next = 3;
    let mut result: i128 = 0;
    let mut previous_1: i128 = 1;
    let mut previous_2: i128 = 1;

    while next <= n{
        result = previous_1 + previous_2;

        previous_2 = previous_1;
        previous_1 = result;

        next += 1;
    }
    return result;
}


fn fib_last_n_digits( n: i32, last: i8) -> i64 {
    if n <= 0{
        return 0;
    }
    if n == 1 || n == 2
    {
        return 1;
    }

    let mut next = 3;
    let mut result: i64 = 0;

    let mut previous_1:i64 = 1;
    let mut previous_2: i64 = 1;

    let ten: i64 = 10;
    let shift: i64  = ten.pow(last as u32);

    while next <= n{
        next += 1;

        result = (previous_1 + previous_2 ).wrapping_rem( shift );
        previous_2 = previous_1;
        previous_1 = result;
    }

    return result;
}

fn test_data() -> [(i32, i128, i32); 27] {
    let data = [
        //(n, fib, last2)
        (0, 0, 0),
        (1, 1, 1),
        (2, 1, 1),
        (3, 2, 2),
        (4, 3, 3),
        (5, 5, 5),
        (6, 8, 8),
        (7, 13, 13),
        (8, 21, 21),
        (9, 34, 34),
        (10, 55, 55),
        (11, 89, 89),
        (12, 144, 44),
        (13, 233, 33),
        (14, 377, 77),
        (15, 610, 10),
        (16, 987, 87),
        (17, 1597, 97),
        (18, 2584, 84),
        (19, 4181, 81),
        (100, 354224848179261915075, 75),
        (150, 9969216677189303386214405760200, 00),
        (175, 1672445759041379840132227567949787325, 25),
        (180, 18547707689471986212190138521399707760, 60),
        (182, 48558529144435440119720805669229197641, 41),
        (183, 78569350599398894027251472817058687522, 22),
        (184, 127127879743834334146972278486287885163, 63)
    ];
    return data;
}

#[test]
fn test_fib() {
    for (n, expected, _) in test_data() {
        if !(n >= 20){
            assert_eq!(fib(n), expected);
        }
    }
}

#[test]
fn test_fib_non_recursive() {
    for (n, expected, _) in test_data() {
        assert_eq!(fib_non_recursive(n), expected);
    }
}

#[test]
fn break_fib_non_recursive(){
    assert_eq!(fib_non_recursive(100), 354224848179261915075);
    assert_eq!(fib_non_recursive(150), 9969216677189303386214405760200);
    assert_eq!(fib_non_recursive(175), 1672445759041379840132227567949787325);
    assert_eq!(fib_non_recursive(180), 18547707689471986212190138521399707760);
    assert_eq!(fib_non_recursive(182), 48558529144435440119720805669229197641);
    assert_eq!(fib_non_recursive(183), 78569350599398894027251472817058687522);
    assert_eq!(fib_non_recursive(184), 127127879743834334146972278486287885163);
    //breaks as too large for i128
    //assert_eq!(fib_non_recursive(185), 127127879743834334146972278486287885163);

}

#[test]
fn test_fib_last_2_digits(){
    for( n, _, expected ) in test_data(){
        assert_eq!(fib_last_n_digits(n, 2) as i32, expected);
    }
}

#[test]
fn test_fib_last_10_digits(){
    //assert_eq!( fib_last_n_digits(100,10), 9261915075 );
    //assert_eq!( fib_last_n_digits(1000,10), 6849228875 );
    assert_eq!( fib_last_n_digits(1000000,10), 8242546875 );
}


