use std::fs;

fn main(){
    let data = read_in_file("data.txt".to_string());
    let mut in_common: Vec<char> = Vec::new();
    let mut data_merged: Vec<String> = Vec::new();
    for pair in data {
        data_merged.push(pair[0].to_string() + &pair[1].to_string());
    }
    for pair in data {
        let common: char = char_in_common(pair[0].to_string(), pair[1].to_string());
        in_common.push(common);
    }
    let mut score: u32 = 0;
    for c in in_common {
        score += c.to_digit(36).unwrap() - 9 + 26*(c.is_uppercase() as u32);
    }
    println!("Score: {}", score);

    
}

fn read_in_file(name: String) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(name)
        .expect("Something went wrong reading the file");
    // Vector of vectors
    let mut lines: Vec<Vec<String>> = Vec::new();

    let mut ctr: u32 = 0;
    // Iterate over the lines
    for line in contents.lines() {
        lines.push(Vec::new());
        let tot_len: u32 = (line.trim().chars().count()/2) as u32;
        lines[ctr as usize].push(line[..tot_len as usize].to_string());
        lines[ctr as usize].push(line[tot_len as usize..].to_string());
        ctr+=1;
    }
    lines
}

fn char_in_common(a: String, b: String) -> char {
    for c in a.chars() {
        if b.contains(c) {
            return c;
        }
    }
    ' '
}

fn chars_in_common(a: String, b: String) -> String {
    let mut common: String;
    for c in a.chars() {
        if b.contains(c) {
            common += c;
        }
    }
    common
}