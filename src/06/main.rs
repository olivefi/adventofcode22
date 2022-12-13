use std::fs;

fn main(){
    let data = read_in_file("data.txt".to_string());
    
    let mut buffer: Vec<char> = Vec::new();
    let mut marker: u32 = 0;
    for (i, c) in data.chars().enumerate() {
        buffer.insert(0, c);
        if buffer.len() == 14 {
            if different_chars(&buffer) {
                marker = i as u32;
                break;
            }
            buffer.pop();
        }
    }
    println!("Marker: {}", marker+1);
    
}

fn read_in_file(name: String) -> String {
    let contents = fs::read_to_string(name)
        .expect("Something went wrong reading the file");
    contents
}

fn different_chars(a: &Vec<char>) -> bool {
    let mut seen: Vec<char> = Vec::new();
    for c in a {
        if seen.contains(c) {
            return false;
        }
        seen.push(*c);
    }
    true
}