use std::fs;

fn main(){
    let data = read_in_file("data.txt".to_string());

    let mut ctr: u32 = 0;
    for (range1, range2) in &data {
        if range1.min >= range2.min && range1.min <= range2.max {
            ctr += 1;
        } else if range2.min <= range1.min && range2.max >= range1.max {
            ctr += 1;
        }
    }
    println!("Contained: {}", ctr);

    let mut overlap: u32 = 0;
    for (range1, range2) in &data {
        if (range1.min >= range2.min && range1.min <= range2.max) || (range1.max >= range2.min && range1.max <= range2.max) {
            overlap += 1;
        } else if (range2.min >= range1.min && range2.min <= range1.max) || (range2.max >= range1.min && range2.max <= range1.max) {
            overlap += 1;
        }
    }
    println!("Overlap: {}", overlap);

}

fn read_in_file(name: String) -> Vec<(Range, Range)> {
    let contents = fs::read_to_string(name)
        .expect("Something went wrong reading the file");
    // Vector of vectors
    let mut data: Vec<(Range, Range)> = Vec::new();

    // Iterate over the lines
    for line in contents.lines() {
        let mut storage: Vec<u32> = Vec::new();

        for el in line.trim().split(",") {
            for nr in el.trim().split("-") {
                storage.push(nr.parse::<u32>().unwrap());
            }
        }
        data.push((Range{min: storage[0], max: storage[1]}, Range{min: storage[2], max: storage[3]}));
    }
    data
}

struct Range {
    min: u32,
    max: u32,
}