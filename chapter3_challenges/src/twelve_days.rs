fn sing(day: usize) -> String {
    let days = [
        ("first", "partridge in a pear tree."),
        ("second", "Two turtle doves,"),
        ("third", "Three french hens,"),
        ("fourth", "Four calling birds,"),
        ("fifth", "FIVE GOLDEN RINGS!"),
        ("sixth", "Six geese a laying,"),
        ("seventh", "Seven swans a swimming,"),
        ("eighth", "Eight maids a milking,"),
        ("ninth", "Nine ladies dancing,"),
        ("tenth", "Ten lords a leaping,"),
        ("eleventh", "Eleven pipers piping,"),
        ("twelfth", "Twelve drummers drumming,")
    ];

    let mut d = day;
    let mut result = format!("On the {} day of Christmas my true love gave to me.", days[d - 1].0);
    while d > 1 {
        d -= 1;
        result = format!("{result}\n{}", days[d].1)
    }
    let last = if day == 1 { "A" } else { "And a" };
    result = format!("{result}\n{last} {}", days[0].1);

    return result;
}

#[test]
fn first_day() {
    let expected = "On the first day of Christmas my true love gave to me.\n\
                         A partridge in a pear tree.";

    assert_eq!(sing(1), expected);
}

#[test]
fn second_day() {
    let expected = "On the second day of Christmas my true love gave to me.\n\
    Two turtle doves,\n\
    And a partridge in a pear tree.";

    assert_eq!(sing(2), expected);
}

#[test]
fn third() {
    let expected = "On the third day of Christmas my true love gave to me.\n\
    Three french hens,\n\
    Two turtle doves,\n\
    And a partridge in a pear tree.";

    assert_eq!(sing(3), expected);
}

#[test]
fn twelfth_day() {
    let expected = "On the twelfth day of Christmas my true love gave to me.\n\
    Twelve drummers drumming,\n\
    Eleven pipers piping,\n\
    Ten lords a leaping,\n\
    Nine ladies dancing,\n\
    Eight maids a milking,\n\
    Seven swans a swimming,\n\
    Six geese a laying,\n\
    FIVE GOLDEN RINGS!\n\
    Four calling birds,\n\
    Three french hens,\n\
    Two turtle doves,\n\
    And a partridge in a pear tree.";

    assert_eq!(sing(12), expected);
}