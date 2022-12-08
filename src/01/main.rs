use std::fs;

fn main() {
    let data = read_in_file("data.txt".to_string());
    let mut total: Vec<u32> = Vec::new();

    // Create vector of total calories
    for dvec in data {
        total.push(0);
        let idx = total.len() - 1;
        for d in dvec {
            total[idx] += d;
        }
    }

    // Find the max
    let mut max = 0;
    for t in &total {
        if *t > max {
            max = *t;
        }
    }

    // Find the top 3
    total.sort_by(|a, b| b.cmp(a));

    println!("Max: {}", total[0]);


    println!("Sum: {}", total[0] + total[1] + total[2]);
}

fn read_in_file(name: String) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(name)
        .expect("Something went wrong reading the file");
    // Vector of vectors
    let mut numbers: Vec<Vec<u32>> = vec![vec![0]];

    let mut elf_number: u32 = 0;
    // Iterate over the lines
    for line in contents.lines() {
        // When we move to a new elf, iterate
        if line == "" {
            elf_number += 1;
            numbers.push(vec![0]);
        } else {
            numbers[elf_number as usize].push(line.parse().unwrap());
        }
    }
    numbers
}