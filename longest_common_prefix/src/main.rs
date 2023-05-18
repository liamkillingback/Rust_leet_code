
//beats 100% runtime
fn main() {
    let solution = longest_common_prefix(vec![String::from("flower"), String::from("flight"), String::from("fling")]);
    println!("{}", solution);
}

fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix: String = String::new();
        let word_length = strs[0].len();

        for i in 0..word_length {
            let letter = strs[0].chars().nth(i);
            let all_same = strs.iter().all(|s| s.chars().nth(i) == letter);
            if all_same {
                prefix.push(letter.unwrap());
            }
            else {
                return prefix;
            }
        }
        prefix
    }
