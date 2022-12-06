use std::collections::HashSet;

pub fn run() {
    let contents = crate::helpers::read_input(6);
    let byte_contents = contents.as_bytes();

    let match_count = 4; // 14 for Part 2

    let mut index = 0;
    for i in  match_count-1..byte_contents.len() {
        
        let mut chars: HashSet<char> = HashSet::new();

        for j in 0..match_count {
            let char = byte_contents[i-j] as char;
            chars.insert(char);
        }

        if chars.len() == match_count {
            index = i + 1;
            break
        }
    }
    println!("{}", index);

}