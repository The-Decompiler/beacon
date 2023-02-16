use std::fs;

fn main() {
    let file_path = "urls.txt";

    let contents = fs::read_to_string(file_path).expect("couldn't find the file");

    let lines = contents.split("\n");

    for line in lines {
        println!("{}", line)
    }
}
