use std::fs;

fn main(){
    let data = read_in_file("data.txt".to_string());
    let mut in_common: Vec<char> = Vec::new();
    let mut data_merged: Vec<String> = Vec::new();

    for pair in data {
        data_merged.push(pair[0].to_string() + &pair[1].to_string());
        let common: char = char_in_common(pair[0].to_string(), pair[1].to_string());
        in_common.push(common);
    }
    let mut score: u32 = 0;
    for c in in_common {
        score += c.to_digit(36).unwrap() - 9 + 26*(c.is_uppercase() as u32);
    }
    println!("Score: {}", score);

    let mut score_groups: u32 = 0;
    let groups: u32 = (data_merged.len()/3) as u32;
    for i in 0..groups {
        let mut common: String = chars_in_common(data_merged[(i*3) as usize].to_string(), data_merged[(i*3+1) as usize].to_string());
        let common_c: char = char_in_common(common, data_merged[(i*3+2) as usize].to_string());
        score_groups += common_c.to_digit(36).unwrap() - 9 + 26*(common_c.is_uppercase() as u32);
    }
    println!("Score Groups: {}", score_groups);

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
    let mut common: String = "".to_string();
    for c in a.chars() {
        if b.contains(c) {
            common += &c.to_string();
        }
    }
    common
}