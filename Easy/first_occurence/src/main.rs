

use std::convert::TryInto;
fn main() {
    let solution = str_str(String::from("sadbutsad"), String::from("sad"));
    println!("{}", solution);
}

fn str_str(haystack: String, needle: String) -> i32 {
    if let Some(index) = haystack.find(&needle) {
       let index_i32: i32 = index.try_into().unwrap();
       return index_i32
    }
    -1
}