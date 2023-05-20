fn main() {
    let solution: bool = is_valid(String::from("()[]{}"));
    println!("{}", solution);
}
fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for char in s.chars() {
        match char {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' if stack.pop() != Some(char) => return false,
            _ => ()
        }
    }    
    stack.is_empty()
    
}