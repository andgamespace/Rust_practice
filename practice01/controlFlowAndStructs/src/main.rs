pub struct CsvParser {
    pub delimiter : char,
    has_headers: bool,
    pub quote_char: char,

    headers: Vec<String>,
}
pub struct column {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool
}
// there are a few different ways to create a struct in rust
//the slice type is a reference to a contiguous sequence of elements 
// of the same type, and it is represented by a poitner to the 
// this is a function that takes a reference to a CsvParser
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    println!("Hello, world!");
    let number = 3;
    if number < 5 {
        println!("my struct Name");
    } else {
        println!("condition was false");
    }
}
