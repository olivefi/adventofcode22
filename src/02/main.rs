use std::fs;

fn main(){
    let data = read_in_file("data.txt".to_string());
    let mut score: u32 = 0;
    let mut elf_score: u32 = 0;
    for line in data {
        let op = line[0].clone();
        let pl = line[1].clone();
        score += clash(op.clone(), pl.clone());
        score += value_of_hand(pl.clone());
        elf_score += desired_result_score(pl.clone());
        elf_score += required_hand_score(op.clone(), pl.clone());
    }
    println!("Score: {}", score);
    println!("Elf Score: {}", elf_score);
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
        for substr in line.trim().split(" ") {
            lines[ctr as usize].push(substr.to_string());
        }
        ctr+=1;
    }
    lines
}

fn desired_result_score(hand: String) -> u32{
    if hand == "X" {
        return 0;
    } else if hand == "Y" {
        return 3;
    } else {
        return 6;
    }
}

fn required_hand_score(op: String, hand: String) -> u32{
    if hand == "X" {
        if op == "A" {
            return 3;
        } else if op == "B" {
            return 1;
        } else {
            return 2;
        }
    } else if hand == "Y" {
        if op == "A" {
            return 1;
        } else if op == "B" {
            return 2;
        } else {
            return 3;
        }
    } else {
        if op == "A" {
            return 2;
        } else if op == "B" {
            return 3;
        } else {
            return 1;
        }
    }
}

fn value_of_hand(hand: String) -> u32 {
    if hand == "X" {
        return 1;
    } else if hand == "Y" {
        return 2;
    } else {
        return 3;
    }
}

fn clash(op: String, pl: String) -> u32 {
    if op == "A".to_string() {
        if pl == "X".to_string() {
            return 3;
        } else if pl == "Y".to_string() {
            return 6;
        } else {
            return 0;
        }
    }
    if op == "B".to_string() {
        if pl == "X".to_string() {
            return 0;
        } else if pl == "Y".to_string() {
            return 3;
        } else {
            return 6;
        }
    }
    if op == "C".to_string() {
        if pl == "X".to_string() {
            return 6;
        } else if pl == "Y".to_string() {
            return 0;
        } else {
            return 3;
        }
    }
    return 0;
} 