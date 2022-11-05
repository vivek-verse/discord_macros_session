use declarative::*;
use std::collections::HashMap;
fn main() {
    let m: HashMap<&str, i32> = vivek![("hey", 1), ("hello", 2), ("there", 3)];
    println!("{m:?}");
    let m2: HashMap<&str, i32> = vivek!["hey" => 1, "hello" => 2];
    println!("{m2:?}");
    let mut m3 = vector_of_tuples!(String, i32);
    m3.push(("vivek".to_owned(), 10));
}
