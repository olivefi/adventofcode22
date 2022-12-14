use std::fs;

fn main(){
    let data = read_in_file("data.txt".to_string());
    let mut root = Directory{name: "root".to_string(), files: Vec::new(), size: 0, directories: Vec::new()};
    let mut current_dir = &mut root;

    for block in data {
        if block.command.starts_with("$ cd") {
            let desired = block.command.split(" ").nth(2).unwrap();

        }
    }
}

fn read_in_file(name: String) -> Vec<Block> {
    let contents = fs::read_to_string(name)
        .expect("Something went wrong reading the file");

    let mut blocks: Vec<Block> = Vec::new();
    let mut block: usize = 0;
    for line in contents.lines() {
        if line.starts_with("$") {
            block += 1;
            blocks.push(Block{command: line.to_string(), output: Vec::new()});
        } else {
            blocks[block - 1].output.push(line.to_string());
        }
    }
    blocks
}

struct Directory{
    name: String,
    files: Vec<File>,
    size: usize,
    directories: Vec<Directory>,
}

struct File{
    name: String,
    size: usize,
}

struct Block{
    command: String,
    output: Vec<String>,
}
