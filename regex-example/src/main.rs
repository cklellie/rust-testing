use regex::Regex;

fn main() {
    let re = Regex::new( r"^\d{4}-\d{2}-\d{2}$" ).unwrap();
    println!("Did our date match? {}", re.is_match( "2021-04-18") );
    println!("Did our date match? {}", re.is_match( "2021-04") );
}

fn check_match( re: Regex, v: &str )-> bool{
    return re.is_match( v );
}

#[test]
fn my_test(){
    let re = Regex::new( r"^\d{4}-\d{2}-\d{2}$" ).unwrap();

    assert!( check_match( re, "2021-04-12" ) );
}


