use std::fs;

fn main(){
    let data = read_in_file("data.txt".to_string());
    let mut crates = data.0;
    let mut crates2 = crates.clone();
    let instructions = data.1;
    
    for ins in instructions {
        crates.move_crates(ins.clone());
        crates2.move_crates_9001(ins.clone());
    }
    for i in 0..9 {
        println!("{:?}, {:?}", crates.crates[i as usize].last().unwrap(), crates2.crates[i as usize].last().unwrap());
    }
    
}

fn read_in_file(name: String) -> (StackedCrates, Vec<Instruction>) {
    let contents = fs::read_to_string(name)
        .expect("Something went wrong reading the file");

    let mut crates: Vec<Vec<char>> = vec![Vec::new(); 9];
    let mut instructions: Vec<Instruction> = Vec::new();
    // Iterate over the lines
    for line in contents.lines() {
        if line.starts_with("move") {
            // Parse the instruction
            let mut ins: Instruction = Instruction {
                from: 0,
                to: 0,
                amount: 0
            };
            let mut split = line.split_whitespace();
            for i in 0..6 {
                let word = split.next().unwrap();
                if i == 1 {
                    ins.amount = word.parse().unwrap();
                } else if i == 3 {
                    ins.from = word.parse().unwrap();
                } else if i == 5 {
                    ins.to = word.parse().unwrap();
                }
            }
            instructions.push(ins);
        } else if line.starts_with("[") {
            let mut next_num = 0;
            for (i, c) in line.chars().enumerate() {
                if c == '[' {
                    next_num = i + 1;
                }
                if i == next_num {
                    crates[(i-1)/4 as usize].insert(0, c);
                }
            }
        }
    }

    (StackedCrates { crates }, instructions)
}

struct StackedCrates {
    crates: Vec<Vec<char>>
}

struct Instruction {
    from: u32,
    to: u32,
    amount: u32,
}

impl Clone for Instruction {
    fn clone(&self) -> Self {
        Instruction {
            from: self.from,
            to: self.to,
            amount: self.amount
        }
    }
}

impl Clone for StackedCrates {
    fn clone(&self) -> Self {
        let mut crates: Vec<Vec<char>> = vec![Vec::new(); 9];
        for i in 0..9 {
            crates[i as usize] = self.crates[i as usize].clone();
        }
        StackedCrates { crates }
    }
}

impl StackedCrates {
    fn move_crates(&mut self, ins: Instruction) {
        let from = ins.from - 1;
        let to = ins.to - 1;
        let amount = ins.amount;

        for _i in 0..amount {
            let c = self.crates[from as usize].pop().unwrap();
            self.crates[to as usize].push(c);
        }
    }

    fn move_crates_9001(&mut self, ins: Instruction) {
        let from = ins.from - 1;
        let to = ins.to - 1;
        let amount = ins.amount;

        let mut temp: Vec<char> = Vec::new();
        for _i in 0..amount {
            let c = self.crates[from as usize].pop().unwrap();
            temp.push(c);
        }
        for _i in 0..amount {
            let c = temp.pop().unwrap();
            self.crates[to as usize].push(c);
        }
    }
}


