use std::fs::File;
use std::io::{ self, BufRead, BufReader };

pub fn read_lines_as_buffer(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines(); 
}

pub fn read_lines(filename: String) -> Vec<String> {
    
    let mut output: Vec<String> = Vec::new();

    let lines = read_lines_as_buffer(filename);

    for line in lines{
        output.push(line.unwrap());
    } 

    return output;
}