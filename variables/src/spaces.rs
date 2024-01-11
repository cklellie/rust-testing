fn spaces() -> usize {
    let spaces = "   ";
    let spaces = spaces.len();
    return spaces;
}

#[test]
fn spaces_test() {
    assert_eq!(spaces(), 3)
}